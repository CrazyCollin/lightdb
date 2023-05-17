use crate::fio::{new_io_manager, IOManager};
use crate::Result;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use crate::data::log_record::ReadLogRecord;

use super::log_record::LogRecord;

pub const DATA_FILE_NAME_SUFFIX: &str = ".data";
pub const HINT_FILE_NAME_SUFFIX:&str="_hint_file";

/// DataFile use to
pub struct DataFile {
    // current file id
    file_id: Arc<RwLock<u64>>,

    // record current file's write offset
    offset: Arc<RwLock<u64>>,

    // use to manage data file io
    io_manager: Box<dyn IOManager>,
}

impl DataFile {
    pub fn new(path: PathBuf, file_id: u64) -> Result<Self> {
        let file_name = path.join(format!("{:09}", file_id) + DATA_FILE_NAME_SUFFIX);
        let io_manager = new_io_manager(file_name)?;
        Ok(Self {
            file_id: Arc::new(RwLock::new(file_id)),
            offset: Arc::new(RwLock::new(0)),
            io_manager: Box::new(io_manager),
        })
    }

    pub fn new_hint_file(path:PathBuf) -> Result<Self> {
        let file_name=path.join(HINT_FILE_NAME_SUFFIX);
        let io_manager=new_io_manager(file_name)?;
        Ok(Self { 
            file_id: Arc::new(RwLock::new(0)), 
            offset: Arc::new(RwLock::new(0)), 
            io_manager: Box::new(io_manager), 
        })
    }

    pub fn get_offset(&self)->u64{
        let read_guard=self.offset.read();
        *read_guard
    }

    pub fn set_offset(&self,offset:u64){
        let mut write_guard=self.offset.write();
        *write_guard=offset
    }

    pub fn get_file_id(&self)->u64{
        let read_guard=self.file_id.read();
        *read_guard
    }
}

impl DataFile {
    // read a log record from a data file
    pub fn read_log_record(&self,offset:u64)->Result<ReadLogRecord> {
        todo!()
    }

    pub fn write(&self,data:&[u8])->Result<usize> {
        let writed_size=self.io_manager.write(data)?;
        let current_offset=self.get_offset();
        self.set_offset(current_offset+writed_size as u64);
        Ok(writed_size)
    }

    pub fn sync(&self)->Result<()> {
        self.io_manager.sync()
    }

}

#[cfg(test)]
mod tests {
    use crate::data::data_file::DataFile;

    #[test]
    fn test_new_data_file(){
        let tmp_path=std::env::temp_dir();

        let file1=DataFile::new(tmp_path.clone(),0);
        assert!(file1.is_ok());
        assert_eq!(file1.unwrap().get_file_id(),0);

        let file2=DataFile::new(tmp_path.clone(),1);
        assert!(file2.is_ok());
        assert_eq!(file2.unwrap().get_file_id(),1);

        let file3=DataFile::new(tmp_path.clone(),2);
        assert!(file3.is_ok());
        assert_eq!(file3.unwrap().get_file_id(),2);
    }
}
