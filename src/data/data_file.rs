use crate::fio::IOManager;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;

/// DataFile use to
pub struct DataFile {
    // current file id
    file_id: Arc<RwLock<u64>>,

    // record current file's write offset
    offset: Arc<RwLock<u64>>,
    io_manager: Box<dyn IOManager>,
}

impl DataFile {}
