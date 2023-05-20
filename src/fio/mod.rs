mod file_io;
mod mmap_io;

use crate::fio::file_io::FileIO;
use crate::Result;
use std::path::PathBuf;
use crate::fio::mmap_io::MmapIO;
use crate::options::IOType;

// receive different io types,
// current support standard file io
pub trait IOManager: Send + Sync {
    // read bytes data from file
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;

    // write bytes to file
    fn write(&self, buf: &[u8]) -> Result<usize>;

    // persist data
    fn sync(&self) -> Result<()>;

    // file size
    fn size(&self) -> u64;
}

pub fn new_io_manager(file_name: PathBuf,io_type:IOType) -> Result<Box<dyn IOManager>> {
    match io_type {
        IOType::StdIO => Ok(Box::new(FileIO::new(file_name)?)),
        IOType::MmapIO => Ok(Box::new(MmapIO::new(file_name)?)),
    }
}
