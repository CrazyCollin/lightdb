use crate::fio::{new_io_manager, IOManager};
use crate::Result;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use bytes::{Buf, BufMut, BytesMut};
use crate::data::log_record::{ReadLogRecord, RecordType};
use crate::errors::Errors;
use crate::options::IOType;

use super::log_record::{LogRecord, LogRecordPos};

const DATA_FILE_NAME_SUFFIX: &str = ".data";
pub const HINT_FILE_NAME_SUFFIX:&str="_hint_file";
const MERGE_FINISHED_FILE_NAME_SUFFIX:&str="_merged_finished_file";
const TXN_SEQ_FILE_NAME_SUFFIX:&str="_txn_seq_file";

/// DataFile use to manage a file which store log record
pub struct DataFile {
    // current file id
    file_id: Arc<RwLock<u64>>,

    // record current file's write offset
    offset: Arc<RwLock<u64>>,

    // use to manage data file io
    io_manager: Box<dyn IOManager>,
}

impl DataFile {
    // create a new data file
    pub fn new(path: PathBuf, file_id: u64,io_type:IOType) -> Result<Self> {
        let file_name = new_file_name(path, file_id);
        let io_manager = new_io_manager(file_name,io_type)?;

        Ok(Self {
            file_id: Arc::new(RwLock::new(file_id)),
            offset: Arc::new(RwLock::new(0)),
            io_manager,
        })
    }

    // create a hint data file
    pub fn new_hint_file(path:PathBuf) -> Result<Self> {
        let file_name=path.join(HINT_FILE_NAME_SUFFIX);
        let io_manager=new_io_manager(file_name,IOType::StdIO)?;

        Ok(Self {
            file_id: Arc::new(RwLock::new(0)),
            offset: Arc::new(RwLock::new(0)),
            io_manager,
        })
    }

    // create a merge finished file
    pub fn new_merge_fin_file(path:PathBuf)->Result<Self>{
        let file_name=path.join(MERGE_FINISHED_FILE_NAME_SUFFIX);
        let io_manager=new_io_manager(file_name,IOType::StdIO)?;

        Ok(Self{
            file_id:Arc::new(RwLock::new(0)),
            offset:Arc::new(RwLock::new(0)),
            io_manager,
        })
    }

    // create a txn seq file
    pub fn new_txn_seq_file(path:PathBuf)->Result<Self>{
        let file_name=path.join(TXN_SEQ_FILE_NAME_SUFFIX);
        let io_manager=new_io_manager(file_name,IOType::StdIO)?;

        Ok(Self{
            file_id:Arc::new(RwLock::new(0)),
            offset:Arc::new(RwLock::new(0)),
            io_manager,
        })
    }

    pub fn get_offset(&self)->u64{
        let read_guard=self.offset.read();
        *read_guard
    }

    pub fn set_offset(&self,offset:u64){
        let mut write_guard=self.offset.write();
        *write_guard=offset
    }

    pub fn get_file_id(&self)->u64{
        let read_guard=self.file_id.read();
        *read_guard
    }

    pub fn get_data_file_size(&self)->u64{
        self.io_manager.size()
    }

    pub fn set_io_manager(&mut self,io_manager:Box<dyn IOManager>){
        self.io_manager=io_manager;
    }
}

impl DataFile {
    // read a log record from a data file
    pub fn read_log_record(&self,offset:u64)->Result<ReadLogRecord> {
        // first create a max header size buffer
        let mut buf =BytesMut::zeroed(LogRecord::max_header_size());
        self.io_manager.read(&mut buf,offset)?;

        // secondly get log record type
        let record_type:RecordType=buf.get_u8().into();

        // thirdly get key size and size, and get actual log record header size
        let key_size=match prost::decode_length_delimiter(&mut buf) {
            Ok(size)=>size,
            Err(e)=>panic!("data file read log record key size err: {}",e),
        };
        let value_size=match prost::decode_length_delimiter(&mut buf) {
            Ok(size)=>size,
            Err(e)=>panic!("data file read log record value size err: {}",e),
        };
        if key_size==0&&value_size==0 {
            return Err(Errors::ReadFileEOF);
        }
        let header_size=1+prost::length_delimiter_len(key_size)+prost::length_delimiter_len(value_size);

        // fourthly set buffer for log record body and push data into it
        let mut body_buf=BytesMut::zeroed(key_size+value_size+4);
        match self.io_manager.read(&mut body_buf,offset+header_size as u64){
            Ok(size)=>if size!=body_buf.len() { panic!("data file read log record body data err"); },
            Err(e)=>panic!("data file read log record body data err: {}",e),
        };

        // finally set log record
        let log_record=LogRecord{
            key:body_buf[0..key_size].to_vec(),
            value:body_buf[key_size..key_size+value_size].to_vec(),
            record_type,
        };

        // check crc value
        body_buf.advance(key_size+value_size);
        let crc_value=body_buf.get_u32();
        if crc_value!=log_record.get_crc() {
            return Err(Errors::CrcCheckError);
        }

        Ok(ReadLogRecord{
            size: header_size+body_buf.len(),
            log_record,
        })
    }

    pub fn write(&self,data:&[u8])->Result<usize> {
        let write_size=self.io_manager.write(data)?;
        let current_offset=self.get_offset();
        self.set_offset(current_offset+write_size as u64);
        Ok(write_size)
    }

    pub fn write_hint_log(&self,key:Vec<u8>,pos:LogRecordPos)->Result<()>{
        let hint_log_record=LogRecord{
            key,
            value:pos.encode(),
            record_type:RecordType::NORMAL,
        };
        self.io_manager.write(&hint_log_record.encode())?;
        Ok(())
    }

    pub fn sync(&self)->Result<()> {
        self.io_manager.sync()
    }

}

fn new_file_name(path:PathBuf,file_id:u64)->PathBuf {
    path.join(format!("{:09}", file_id) + DATA_FILE_NAME_SUFFIX)
}

#[cfg(test)]
mod tests {
    use crate::data::data_file::{DataFile, new_file_name};
    use crate::data::log_record::{LogRecord, RecordType};
    use crate::options::IOType;

    #[test]
    fn test_new_data_file(){
        let tmp_path=std::env::temp_dir();

        let file1=DataFile::new(tmp_path.clone(),0,IOType::StdIO);
        assert!(file1.is_ok());
        assert_eq!(file1.unwrap().get_file_id(),0);

        let file2=DataFile::new(tmp_path.clone(),1,IOType::StdIO);
        assert!(file2.is_ok());
        assert_eq!(file2.unwrap().get_file_id(),1);

        let file3=DataFile::new(tmp_path.clone(),2,IOType::StdIO);
        assert!(file3.is_ok());
        assert_eq!(file3.unwrap().get_file_id(),2);
    }

    #[test]
    fn test_read_log_record_from_data_file(){
        let temp_path=std::env::temp_dir();
        let data_file=DataFile::new(temp_path.clone(),0,IOType::StdIO).unwrap();
        println!("{}",temp_path.to_str().unwrap());
        let log_record=LogRecord{
            key: "key".into(),
            value: "value".into(),
            record_type: RecordType::NORMAL,
        };
        let encoded_data=log_record.encode();
        let write_size=data_file.write(&encoded_data).unwrap();
        println!("{}",write_size);

        let read_log_record=data_file.read_log_record(0);
        println!("{:?}",read_log_record.unwrap());
        std::fs::remove_file(new_file_name(temp_path,0)).unwrap()
    }
}
