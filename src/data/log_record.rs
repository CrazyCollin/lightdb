pub struct LogRecord {
    pub(crate) key: Vec<u8>,
    pub(crate) value: Vec<u8>,
    pub(crate) record_type: RecordType,
}

#[derive(PartialEq)]
pub enum RecordType {
    NORMAL = 1,
    DELETED = 2,
}

/// LogRecordPos shows log record position
#[derive(Copy, Clone, Debug)]
pub struct LogRecordPos {
    pub(crate) file_id: u64,
    pub(crate) offset: u64,
}

impl LogRecord {}
