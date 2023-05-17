use crate::data::data_file::DataFile;
use crate::data::log_record::{LogRecord, LogRecordPos, RecordType};
use crate::errors::Errors;
use crate::index::{Index, IndexIterator};
use crate::options::{IteratorOptions, Options};
use crate::Result;
use bytes::Bytes;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

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
    key_counts:usize,

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

    pub(crate) fn append_log_record(&self,log_record:&mut LogRecord)->Result<LogRecordPos>{

        todo!()
    }
}

// compaction related
impl Engine {}

pub struct Iterator<'a> {
    index_iterator: Arc<RwLock<Box<dyn IndexIterator>>>,
    engine: &'a Engine,
}

impl Iterator<'_> {
    pub fn new()->Self{
        todo!()
    }
}

impl Engine {
    pub fn iter(&self, options: IteratorOptions) -> Iterator {
        Iterator {
            index_iterator: Arc::new(RwLock::new(self.index.iterator(options))),
            engine: self,
        }
    }

    pub fn list_keys(&self) -> Result<Vec<Bytes>> {
        match self.index.list_keys() {
            Some(keys) => Ok(keys),
            None => Err(Errors::KeyNotFound),
        }
    }
}

#[cfg(test)]
mod tests {

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

    fn create_db() {}
}
