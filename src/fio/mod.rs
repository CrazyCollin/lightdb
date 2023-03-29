mod file_io;

use crate::fio::file_io::FileIO;
use crate::Result;
use std::path::PathBuf;

// receive different io types,
// current support standard file io
pub trait IOManager: Send + Sync {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;

    fn write(&self, buf: &[u8]) -> Result<usize>;

    fn sync(&self) -> Result<()>;
}

pub fn new_io_manager(file_name: PathBuf) -> Result<impl IOManager> {
    FileIO::new(file_name)
}
