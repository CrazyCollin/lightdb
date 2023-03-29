use crate::data::log_record::LogRecordPos;

mod btree;

pub trait Indexer: Sync + Send {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos>;

    fn delete(&self, key: Vec<u8>) -> bool;
}
