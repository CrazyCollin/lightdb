use std::sync::Arc;
use bytes::Bytes;
use crossbeam_skiplist::{SkipMap};
use crate::data::log_record::LogRecordPos;
use crate::index::{Index, IndexIterator};
use crate::options::IteratorOptions;

pub struct SkipListIndex{
    index:Arc<SkipMap<Vec<u8>,LogRecordPos>>,
}

impl SkipListIndex {
    pub fn new()->Self{
        Self{
            index:Arc::new(SkipMap::new()),
        }
    }
}

impl Index for SkipListIndex {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool {
        todo!()
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        todo!()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        todo!()
    }

    fn list_keys(&self) -> Option<Vec<Bytes>> {
        todo!()
    }

    fn iterator(&self, options: IteratorOptions) -> Box<dyn IndexIterator> {
        todo!()
    }
}

pub struct SkipListIndexIterator{
    items:Vec<(Vec<u8>,LogRecordPos)>,
    index:usize,
    options:IteratorOptions,
}

impl IndexIterator for SkipListIndexIterator {
    fn rewind(&mut self) {
        self.index=0;
    }

    fn seek(&mut self, key: Vec<u8>) {
        let result=self.items.binary_search_by(|(x,_)|{
            match self.options.reverse {
                true=>{
                    x.cmp(&key).reverse()
                },
                false=>{
                    x.cmp(&key)
                }
            }
        });
        self.index=match result {
            Ok(equal_value)=>equal_value,
            Err(insert_value)=>insert_value,
        }
    }

    fn next(&mut self) -> Option<(Vec<u8>, LogRecordPos)> {
        if self.index>=self.items.len() {
            return None;
        }
        while let Some(item) = self.items.get(self.index) {
            self.index+=1;
            let prefix=&self.options.prefix;
            if prefix.is_empty()||item.0.starts_with(prefix) {
                return Some((item.0.clone(),item.1));
            }
        }
        None
    }
}