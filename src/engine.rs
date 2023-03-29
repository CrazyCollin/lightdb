use crate::data::data_file::DataFile;
use crate::index::Indexer;
use crate::options::Options;
use crate::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use bytes::Bytes;

pub struct Engine {
    options: Arc<Options>,

    active_file: Arc<RwLock<DataFile>>,
    inactive_files: Arc<RwLock<HashMap<u64, DataFile>>>,
    // memory index
    pub(crate) index: Box<dyn Indexer>,
}

impl Engine {
    pub fn open(options:Options)->Result<Self>{
        todo!()
    }

    pub fn close()->Result<()>{
        todo!()
    }
}

impl Engine {
    pub fn get(&self,key:Bytes)->Result<()>{
        todo!()
    }

    pub fn put(&self,key:Bytes,value:Bytes)->Result<()>{
        todo!()
    }

    pub fn remove(&self,key:Bytes)->Result<()>{
        todo!()
    }
}

#[cfg(test)]
mod tests{

    #[test]
    fn test_open_db(){

    }

    #[test]
    fn test_close_db(){

    }

    fn create_db(){

    }

}