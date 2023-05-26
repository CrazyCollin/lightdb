use std::fs::OpenOptions;
use std::path::PathBuf;
use std::sync::Arc;
use log::error;
use memmap2::Mmap;
use parking_lot::{RwLock};
use crate::errors::Errors;
use crate::fio::IOManager;
use crate::Result;

#[derive(Debug)]
pub struct MmapIO {
    mmap:Arc<RwLock<Mmap>>
}

impl MmapIO {
    pub fn new(file_name:PathBuf)->Result<Self>{
        return match OpenOptions::new().
            create(true).
            read(true).
            write(true).
            open(file_name)
        {
            Ok(file) => {
                let map = unsafe {
                    Mmap::map(&file).expect("map file error")
                };
                Ok(Self {
                    mmap: Arc::new(RwLock::new(map)),
                })
            },
            Err(e) => {
                error!("open mmap file err: {}",e);
                Err(Errors::OpenFileError)
            }
        }
    }
}

impl IOManager for MmapIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let read_guard=self.mmap.read();
        let end_offset=offset+buf.len() as u64;
        if end_offset>read_guard.len() as u64 {
            return Err(Errors::ReadFileEOF)
        }
        let read_data=&read_guard[offset as usize..end_offset as usize];
        buf.copy_from_slice(read_data);

        Ok(buf.len())
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    fn sync(&self) -> Result<()> {
        unimplemented!()
    }

    fn size(&self) -> u64 {
        let read_guard=self.mmap.read();
        read_guard.len() as u64
    }
}

#[cfg(test)]
mod tests{
    use std::fs;
    use std::path::PathBuf;

    use crate::fio::file_io::FileIO;
    use crate::fio::IOManager;
    use crate::fio::mmap_io::MmapIO;

    #[test]
    fn test_read(){
        let file_path=PathBuf::from("/tmp/mmap.data");

        write_data(&file_path);

        let mmap_io=MmapIO::new(file_path.clone());
        if mmap_io.is_err() {
            remove_tmp_file(file_path);
            panic!("{:?}",mmap_io.unwrap_err())
        }
        assert!(mmap_io.is_ok());
        let mmap_io=mmap_io.unwrap();
        let mut buf=[0u8;7];
        let read_size=mmap_io.read(&mut buf,0);
        assert!(read_size.is_ok());
        assert_eq!(read_size.unwrap(),7);

        assert!(remove_tmp_file(file_path))
    }

    fn write_data(path:&PathBuf){
        let file_io=FileIO::new(path.clone()).unwrap();
        let write_size=file_io.write(b"abcdefg");
        assert!(write_size.is_ok());
        assert_eq!(write_size.unwrap(),7);
    }

    fn remove_tmp_file(path: PathBuf) -> bool {
        let remove_res = fs::remove_file(path);
        remove_res.is_ok()
    }
}