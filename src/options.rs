use std::path::PathBuf;

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
