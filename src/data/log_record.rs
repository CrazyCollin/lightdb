use bytes::{BufMut, BytesMut};

/// LogRecord use to record key value data into disk
#[derive(Debug)]
pub struct LogRecord {
    pub(crate) key: Vec<u8>,
    pub(crate) value: Vec<u8>,
    pub(crate) record_type: RecordType,
}

#[derive(Debug)]
pub struct ReadLogRecord{
    pub(crate) log_record:LogRecord,
    pub(crate) length:usize,
}

pub struct TxnRecord{
    pub(crate) record:LogRecord,
    pub(crate) position:LogRecordPos,
}

#[derive(PartialEq,Copy, Clone,Debug)]
pub enum RecordType {
    NORMAL = 1,
    DELETED = 2,
}

impl From<u8> for RecordType {
    fn from(value: u8) -> Self {
        match value {
            1=>RecordType::NORMAL,
            2=>RecordType::DELETED,
            _=>panic!("wrong record type!"),
        }
    }
}

/// LogRecordPos shows log record position
#[derive(Copy, Clone, Debug)]
pub struct LogRecordPos {
    // file record location
    pub(crate) file_id: u64,
    // record offset
    pub(crate) offset: u64,
}

impl LogRecordPos {
    pub fn encode(&self)->Vec<u8>{
        let mut buf=BytesMut::new();
        prost::encoding::encode_varint(self.file_id,&mut buf);
        prost::encoding::encode_varint(self.offset,&mut buf);
        buf.to_vec()
    }

    pub fn decode(encoded_data:Vec<u8>)->Self{
        let mut buf=BytesMut::from(encoded_data.as_slice());

        let file_id=match prost::encoding::decode_varint(&mut buf){
            Ok(fid)=>fid,
            Err(e)=>panic!("decode log record pos err: {}",e),
        };
        let offset=match prost::encoding::decode_varint(&mut buf){
            Ok(offset)=>offset,
            Err(e)=>panic!("decode log record pos err: {}",e),
        };
        Self{
            file_id,
            offset,
        }
    }
}

impl LogRecord {
    pub fn encode(&self)->Vec<u8>{
        let (encoded_data,_)=self.internal_encode();
        encoded_data
    }

    pub fn get_crc(&self)->u32{
        let (_,crc)=self.internal_encode();
        crc
    }

    //
    //	+-------------+--------------+-------------+--------------+-------------+-------------+
    //	| record type |    key size  |  value size |     key      |    value    |  crc value  |
    //	+-------------+--------------+-------------+--------------+-------------+-------------+
    // log record encode layout
    pub fn internal_encode(&self)->(Vec<u8>,u32) {
        let mut buf=BytesMut::new();
        buf.reserve(
            std::mem::size_of::<u8>()
                +prost::length_delimiter_len(self.key.len())
                +prost::length_delimiter_len(self.value.len())
                +self.key.len()
                +self.value.len()
        );

        buf.put_u8(self.record_type as u8);

        let encoded_res=prost::encode_length_delimiter(self.key.len(),&mut buf);
        if let Err(e) = encoded_res {
            panic!("{}",e);
        }
        let encoded_res=prost::encode_length_delimiter(self.value.len(),&mut buf);
        if let Err(e) = encoded_res {
            panic!("{}",e);
        }

        buf.extend_from_slice(&self.key);
        buf.extend_from_slice(&self.value);

        let mut hasher=crc32fast::Hasher::new();
        hasher.update(&buf);
        let crc=hasher.finalize();
        buf.put_u32(crc);

        (buf.to_vec(),crc)
    }
}

#[cfg(test)]
mod tests{
    use bytes::BytesMut;

    use crate::data::log_record::{LogRecord, LogRecordPos, RecordType};

    #[test]
    fn test_encode_log_record(){
        // normal situation
        let log_record=LogRecord{
            key: "key".as_bytes().to_vec(),
            value: "value".as_bytes().to_vec(),
            record_type: RecordType::NORMAL,
        };
        let encoded_data=log_record.encode();
        assert_eq!(encoded_data.len(),15);
        assert_eq!(encoded_data[0],RecordType::NORMAL as u8);
        assert_eq!(prost::decode_length_delimiter(&encoded_data[1..]).unwrap(),3);
        assert_eq!(prost::decode_length_delimiter(&encoded_data[2..]).unwrap(),5);
        assert_eq!(&encoded_data[3..6],"key".as_bytes());
        assert_eq!(&encoded_data[6..11],"value".as_bytes());

        // empty value situation
        let log_record=LogRecord{
            key: "key".as_bytes().to_vec(),
            value: Default::default(),
            record_type: RecordType::NORMAL,
        };
        let encoded_data=log_record.encode();
        assert_eq!(encoded_data.len(),10);
        assert_eq!(encoded_data[0],RecordType::NORMAL as u8);
        assert_eq!(prost::decode_length_delimiter(&encoded_data[1..]).unwrap(),3);
        assert_eq!(prost::decode_length_delimiter(&encoded_data[2..]).unwrap(),0);
        assert_eq!(&encoded_data[3..6],"key".as_bytes());

        // big key situation
        let big_key = vec![0u8; 1024];
        let log_record = LogRecord {
            key: big_key.clone(),
            value: "value".as_bytes().to_vec(),
            record_type: RecordType::NORMAL,
        };
        let encoded_data = log_record.encode();
        assert_eq!(encoded_data[0], RecordType::NORMAL as u8);
        assert_eq!(prost::decode_length_delimiter(&encoded_data[1..]).unwrap(), big_key.len());
        assert_eq!(&encoded_data[3..1027], big_key.as_slice());
        // record deleted situation
    }

    #[test]
    fn test_encode_log_record_pos(){
        let log_record_pos=LogRecordPos{
            file_id: 0,
            offset: 10,
        };
        let mut encoded_data=BytesMut::from(log_record_pos.encode().as_slice());
        assert_eq!(encoded_data.len(),2);
        assert_eq!(prost::encoding::decode_varint(&mut encoded_data).unwrap(),0);
        assert_eq!(prost::encoding::decode_varint(&mut encoded_data).unwrap(),10);

        let log_record_pos=LogRecordPos{
            file_id: 0,
            offset: 256,
        };
        let mut encoded_data=BytesMut::from(log_record_pos.encode().as_slice());
        assert_eq!(encoded_data.len(),3);
        assert_eq!(prost::encoding::decode_varint(&mut encoded_data).unwrap(),0);
        assert_eq!(prost::encoding::decode_varint(&mut encoded_data).unwrap(),256);

        let log_record_pos=LogRecordPos{
            file_id: 0,
            offset: 127,
        };
        let mut encoded_data=BytesMut::from(log_record_pos.encode().as_slice());
        assert_eq!(encoded_data.len(),2);
        assert_eq!(prost::encoding::decode_varint(&mut encoded_data).unwrap(),0);
        assert_eq!(prost::encoding::decode_varint(&mut encoded_data).unwrap(),127);
    }

    #[test]
    fn test_decode_log_record_pos() {
        let log_record_pos=LogRecordPos{
            file_id: 0,
            offset: 10,
        };
        let encoded_data=BytesMut::from(log_record_pos.encode().as_slice());

        let decoded_log_record_pos=LogRecordPos::decode(encoded_data.into());
    
        assert_eq!(decoded_log_record_pos.file_id,log_record_pos.file_id);
        assert_eq!(decoded_log_record_pos.offset,log_record_pos.offset);
    }
}
