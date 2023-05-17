use bytes::Bytes;
use crate::data::log_record::LogRecordPos;
use crate::index::btree::BTreeIndex;
use crate::options::{IndexType, IteratorOptions};

mod btree;
mod b_plus_tree;
mod skiplist;

pub trait Index: Sync + Send {
    // put key to into index
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;

    // get value's position in data file
    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos>;

    // delete specific key value pair in index
    fn delete(&self, key: Vec<u8>) -> bool;

    // list all keys
    fn list_keys(&self)->Option<Vec<Bytes>>;

    // get a iterator of current index
    fn iterator(&self,options:IteratorOptions)->Box<dyn IndexIterator>;
}

pub fn new_index(index_type:IndexType)->Box<dyn Index>{
    match index_type {
        IndexType::BTree=>Box::new(BTreeIndex::new()),
        _=>Box::new(BTreeIndex::new()),
    }
}

pub trait IndexIterator:Sync+Send{
    // reset cursor to the beginning of iterator
    fn rewind(&mut self);

    // reset cursor to specific `key` position
    fn seek(&mut self,key:Vec<u8>);

    // return key value pair of current index
    fn next(&mut self)->Option<(Vec<u8>,LogRecordPos)>;
}