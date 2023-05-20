use std::path::PathBuf;
use serde::{Deserialize,Serialize};

#[derive(Clone)]
pub struct Options {
    // database file path
    pub path: PathBuf,

    // data file size
    pub data_file_size: u64,

    // sync after every write
    pub sync_write: bool,

    // index type
    pub index_type: IndexType,
}

#[derive(Clone)]
pub enum IndexType {
    BTree,
    BPlusTree,
    SkipList,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            path: std::env::temp_dir().join("lightkv"),
            data_file_size: 256 * 1024 * 1024,
            sync_write: false,
            index_type: IndexType::BTree,
        }
    }
}

pub struct IteratorOptions{
    pub prefix:Vec<u8>,
    pub reverse:bool,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct ServerConfig{
    pub general_config:GeneralConfig,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct ClientConfig{
    pub general_config:GeneralConfig,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct GeneralConfig{
    pub addr:String,
    pub network:NetworkType,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum NetworkType{
    Tcp,
    Quic,
}

#[derive(Copy, Clone)]
pub enum IOType{
    StdIO,
    MmapIO,
}