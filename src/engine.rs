use crate::data::data_file::DataFile;
use crate::data::log_record::{LogRecord, LogRecordPos, RecordType};
use crate::errors::Errors;
use crate::index::{Index, IndexIterator};
use crate::options::{IteratorOptions, Options,WriteBatchOptions, IOType};
use crate::Result;
use bytes::Bytes;
use parking_lot::{RwLock, Mutex};
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Storage engine instance
pub struct Engine {
    options: Arc<Options>,

    active_file: Arc<RwLock<DataFile>>,
    inactive_files: Arc<RwLock<HashMap<u64, DataFile>>>,
    // memory index
    pub(crate) index: Box<dyn Index>,

    pub(crate) txn_id:Arc<AtomicUsize>,

    file_lock:File,
    written_bytes:Arc<AtomicUsize>,
}

/// Status of engine instance
#[derive(Debug)]
pub struct EngineStatus{
    // engine key counts
    key_counts:usize,

    // engine data files 
    file_counts:usize,

}

impl Engine {
    pub fn open(options: Options) -> Result<Self> {
        // open a engine can be divided into several steps
        // 1.Check engine options
        // 2.Load data files
        // 3.Load index from hint file

        if let Some(e) = Engine::check_options(&options) {
            return Err(e);
        }



        Err(Errors::DataFileSizeError)
    }

    pub fn close(&self) -> Result<()> {
        let read_record = self.active_file.read();
        read_record.sync()
    }

    fn check_options(options: &Options) -> Option<Errors> {
        let path = options.path.to_str();
        if path.is_none() || path.unwrap().is_empty() {
            return Some(Errors::PathEmpty);
        }
        if options.data_file_size == 0 {
            return Some(Errors::DataFileSizeError);
        }
        None
    }
}

impl Engine {
    pub fn get(&self, key: Bytes) -> Result<Bytes> {
        let record_pos = self.index.get(key.to_vec());
        match record_pos {
            Some(pos) => self.get_value_on_offset(pos),
            None => Err(Errors::KeyNotFound),
        }
    }

    pub fn put(&self, key: Bytes, value: Bytes) -> Result<()> {
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }
        let mut log_record=LogRecord{
            key: key.clone().into(),
            value: value.into(),
            record_type: RecordType::NORMAL,
        };

        let log_record_pos=self.append_log_record(&mut log_record)?;
        match self.index.put(key.into(), log_record_pos) {
            true=>Ok(()),
            false=>Err(Errors::IndexUpdateError),
        }
    }

    pub fn remove(&self, key: Bytes) -> Result<()> {
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }

        match self.index.get(key.clone().into()) {
            Some(log_record_pos)=>log_record_pos,
            None => { return Ok(()); }
        };

        let mut log_record=LogRecord{
            key: key.clone().into(),
            value: Default::default(),
            record_type: RecordType::DELETED,
        };

        let _=self.append_log_record(&mut log_record)?;
        match self.index.delete(key.into()) {
            true=>Ok(()),
            false=>Err(Errors::IndexUpdateError),
        }
    }
}

impl Engine {
    // get specific value through value's position
    pub(crate) fn get_value_on_offset(&self, record_pos: LogRecordPos) -> Result<Bytes> {
        let active_file = self.active_file.read();
        let old_file = self.inactive_files.read();
        let log_record = match active_file.get_file_id() == record_pos.file_id {
            true => active_file.read_log_record(record_pos.offset)?,
            false => {
                // get specific data file
                let data_file = old_file.get(&record_pos.offset);
                match data_file {
                    Some(file) => file.read_log_record(record_pos.offset)?,
                    None => {
                        return Err(Errors::DataFileNotFound);
                    }
                }
            }
        };
        let log_record = log_record.log_record;
        if log_record.record_type == RecordType::DELETED {
            return Err(Errors::KeyNotFound);
        }
        Ok(Bytes::from(log_record.value))
    }

    // append log record to active datafile
    pub(crate) fn append_log_record(&self,log_record:&mut LogRecord)->Result<LogRecordPos>{
        let mut active_file=self.active_file.write();

        let encoded_record=log_record.encode();
        let encoded_len=encoded_record.len();
    
        // check if current active datafile size exceed max file size limit
        // create new datafile if exceed
        if active_file.get_data_file_size()+encoded_len as u64>self.options.data_file_size {
            let active_file_id=active_file.get_file_id();

            // insert into old datafile maps
            let mut write_guard=self.inactive_files.write();
            write_guard.insert(active_file_id,DataFile::new(self.options.path.clone(), active_file_id, IOType::StdIO)?);

            let new_active_file=DataFile::new(self.options.path.clone(),active_file_id,IOType::StdIO)?;
            *active_file=new_active_file;
        }

        let offset=active_file.get_offset();
        let write_size=active_file.write(&encoded_record)?;

        let previous_write_bytes=self.written_bytes.fetch_add(write_size, Ordering::SeqCst);

        // divided into 2 cases
        // 1.Enable every sync write
        // 2.Disable every sync write but sync datafile depend on totoal write bytes size
        let mut sync_write=self.options.sync_write;
        if !sync_write&&(previous_write_bytes+write_size)>=self.options.sync_bytes_write {
            sync_write=true;
        }

        if sync_write {
            active_file.sync()?;
            self.written_bytes.store(0, Ordering::SeqCst);
        }
        Ok(LogRecordPos { file_id: active_file.get_file_id(), offset, size: write_size as u64 })
    }
}

// compaction related
impl Engine {

}

pub struct Iterator<'a> {
    index_iterator: Arc<RwLock<Box<dyn IndexIterator>>>,
    engine: &'a Engine,
}

impl<'a> Iterator<'a> {
    pub fn rewind(&self) {
        let mut write_guard=self.index_iterator.write();
        write_guard.rewind();
    }

    pub fn seek(&self,key:Vec<u8>) {
        let mut write_guard=self.index_iterator.write();
        write_guard.seek(key);
    }

    pub fn next(&self)->Option<(Bytes,Bytes)> {
        let mut write_guard=self.index_iterator.write();
        while let Some((key,log_record_pos))=write_guard.next() {
            let value=self.engine.get_value_on_offset(log_record_pos).expect("get value from active file error");
            
            return Some((Bytes::from(key),value));
        }

        None
    }
}

impl Engine {
    pub fn iter(&self, options: IteratorOptions) -> Iterator {
        Iterator {
            index_iterator: Arc::new(RwLock::new(self.index.iterator(options))),
            engine: self,
        }
    }

    pub fn fold<F>(&self,f:F)->Result<()>
    where
        Self:Sized,
        F:Fn(Bytes,Bytes)->bool,
    {
        let iter=self.iter(IteratorOptions::default());

        while let Some((k,v)) = iter.next() {
            if !f(k,v) {
                break;
            }
        }
        Ok(())
    }

    pub fn list_keys(&self) -> Result<Vec<Bytes>> {
        match self.index.list_keys() {
            Some(keys) => Ok(keys),
            None => Err(Errors::KeyNotFound),
        }
    }
}

// transaction related
pub struct WriteBatch<'a>{
    // stash write into pending queue
    pending_writes:Arc<Mutex<HashMap<Vec<u8>,LogRecord>>>,
    engine:&'a Engine,
    options:WriteBatchOptions,
}

impl WriteBatch<'_> {
    pub fn put(&self,key:Bytes,value:Bytes)->Result<()> {
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }

        let log_record=LogRecord{
            key:key.to_vec(),
            value:value.to_vec(),
            record_type:RecordType::NORMAL,
        };

        let mut write_guard=self.pending_writes.lock();
        write_guard.insert(key.to_vec(), log_record);
        Ok(())
    }

    pub fn delete(&self,key:Bytes)->Result<()> {
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }

        let mut write_guard=self.pending_writes.lock();
        write_guard.remove(&key.to_vec());
        Ok(())
    }

    pub fn commit(&self)->Result<()> {

        Ok(())
    }
}

impl Engine {
    fn new_write_batch(){

    }
}

#[cfg(test)]
mod engine_tests {

    #[test]
    fn test_open_db() {}

    #[test]
    fn test_close_db() {}

    #[test]
    fn test_get(){}

    #[test]
    fn test_put(){}

    #[test]
    fn test_remove(){

    }

    #[test]
    fn test_read_log_record_with_pos() {
        
    }

    #[test]
    fn test_append_log_record() {
        
    }

    fn create_db() {}
}

#[cfg(test)]
mod transaction_tests{
    #[test]
    fn test_commit() {
        
    }
}

#[cfg(test)]
mod iterator_tests{
    #[test]
    fn test_list_keys() {
        
    }

    #[test]
    fn test_seek() {
        
    }

    #[test]
    fn test_rewind() {
        
    }

    #[test]
    fn test_next() {
        
    }

    #[test]
    fn test_fold() {
        
    }
}

#[cfg(test)]
mod compaction_tests{

}