use crate::errors::{Errors, Result};
use crate::fio::IOManager;
use log::error;
use parking_lot::RwLock;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::sync::Arc;

pub struct FileIO {
    fd: Arc<RwLock<File>>,
}

impl FileIO {
    pub fn new(file_name: PathBuf) -> Result<Self> {
        match OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(file_name)
        {
            Ok(file) => Ok(Self {
                fd: Arc::new(RwLock::new(file)),
            }),
            Err(e) => {
                error!("failed to open file: {}", e);
                Err(Errors::OpenFileError)
            }
        }
    }
}

impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let read_guard = self.fd.read();
        match read_guard.read_at(buf, offset) {
            Ok(len) => Ok(len),
            Err(e) => {
                error!("read file err: {}", e);
                Err(Errors::ReadFileError)
            }
        }
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut write_guard = self.fd.write();
        match write_guard.write(buf) {
            Ok(len) => Ok(len),
            Err(e) => {
                error!("write file err: {}", e);
                Err(Errors::WriteFileError)
            }
        }
    }

    fn sync(&self) -> Result<()> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("sync file err: {}", e);
            return Err(Errors::SyncFileError);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fio::file_io::FileIO;
    use crate::fio::IOManager;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_read() {
        let file_path = PathBuf::from("/tmp/a.data");
        let file_io = FileIO::new(file_path.clone());
        assert!(file_io.is_ok());
        let file_io = file_io.unwrap();

        let write_size1 = file_io.write("test-1".as_bytes());
        assert!(write_size1.is_ok());
        assert_eq!(write_size1.ok(), Some(6));

        let write_size2 = file_io.write("test-22".as_bytes());
        assert!(write_size2.is_ok());
        assert_eq!(write_size2.ok(), Some(7));

        let write_size3 = file_io.write("test-333".as_bytes());
        assert!(write_size3.is_ok());
        assert_eq!(write_size3.ok(), Some(8));

        let mut read_data1 = [0u8; 6];
        let read_size1 = file_io.read(&mut read_data1, 0);
        assert!(read_size1.is_ok());
        assert_eq!(read_size1.ok(), Some(6));
        let mut read_data2 = [0u8; 7];
        let read_size2 = file_io.read(&mut read_data2, 6);
        assert!(read_size2.is_ok());
        assert_eq!(read_size2.ok(), Some(7));
        let mut read_data3 = [0u8; 8];
        let read_size3 = file_io.read(&mut read_data3, 13);
        assert!(read_size3.is_ok());
        assert_eq!(read_size3.ok(), Some(8));

        let remove_res = remove_tmp_file(&file_path);
        assert!(remove_res);
    }

    #[test]
    fn test_write() {
        let file_path = PathBuf::from("/tmp/b.data");
        let file_io = FileIO::new(file_path.clone());
        assert!(file_io.is_ok());
        let file_io = file_io.unwrap();

        let write_size1 = file_io.write("test-1".as_bytes());
        assert!(write_size1.is_ok());
        assert_eq!(write_size1.ok(), Some(6));

        let write_size2 = file_io.write("test-22".as_bytes());
        assert!(write_size2.is_ok());
        assert_eq!(write_size2.ok(), Some(7));

        let write_size3 = file_io.write("test-333".as_bytes());
        assert!(write_size3.is_ok());
        assert_eq!(write_size3.ok(), Some(8));

        let remove_res = remove_tmp_file(&file_path);
        assert!(remove_res);
    }

    #[test]
    fn test_sync() {
        let file_path = PathBuf::from("/tmp/c.data");
        let file_io = FileIO::new(file_path.clone());
        assert!(file_io.is_ok());
        let file_io = file_io.unwrap();

        let write_size1 = file_io.write("test-1".as_bytes());
        assert!(write_size1.is_ok());
        assert_eq!(write_size1.ok(), Some(6));

        let write_size2 = file_io.write("test-22".as_bytes());
        assert!(write_size2.is_ok());
        assert_eq!(write_size2.ok(), Some(7));

        let write_size3 = file_io.write("test-333".as_bytes());
        assert!(write_size3.is_ok());
        assert_eq!(write_size3.ok(), Some(8));

        let sync_res = file_io.sync();
        assert!(sync_res.is_ok());

        let remove_res = remove_tmp_file(&file_path);
        assert!(remove_res);
    }

    fn remove_tmp_file(path: &PathBuf) -> bool {
        let remove_res = fs::remove_file(path);
        remove_res.is_ok()
    }
}
