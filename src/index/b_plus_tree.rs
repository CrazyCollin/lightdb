use crate::{
    index::{
        Index
    }, options::IteratorOptions
};

pub struct BPlusTreeIndex{
    
}

impl BPlusTreeIndex {
    pub fn new()->Self{
        Self {  }
    }
}

impl Index for BPlusTreeIndex {
    fn put(&self, key: Vec<u8>, pos: crate::data::log_record::LogRecordPos) -> bool {
        todo!()
    }

    fn get(&self, key: Vec<u8>) -> Option<crate::data::log_record::LogRecordPos> {
        todo!()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        todo!()
    }

    fn list_keys(&self)->Option<Vec<bytes::Bytes>> {
        todo!()
    }

    fn iterator(&self,options:crate::options::IteratorOptions)->Box<dyn super::IndexIterator> {
        todo!()
    }
}

pub struct BPlusTreeIndexIterator{
    
    index:usize,
    options:IteratorOptions,
}