use crate::data::log_record::LogRecordPos;
use crate::index::Indexer;
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct BTreeIndex {
    index: Arc<RwLock<BTreeMap<Vec<u8>, LogRecordPos>>>,
}

impl BTreeIndex {
    pub fn new() -> Self {
        Self {
            index: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl Indexer for BTreeIndex {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool {
        let mut write_guard = self.index.write();
        write_guard.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        let read_guard = self.index.read();
        read_guard.get(&key).copied()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        let mut write_guard = self.index.write();
        let res = write_guard.remove(&key);
        res.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::log_record::LogRecordPos;
    use crate::index::btree::BTreeIndex;
    use crate::index::Indexer;

    #[test]
    fn test_put() {
        let btree_index = BTreeIndex::new();

        let test_data = vec![
            ("test-1".into(), LogRecordPos { file_id: 0, offset: 10 }),
            ("test-2".into(), LogRecordPos { file_id: 0, offset: 20 }),
            ("test-3".into(), LogRecordPos { file_id: 0, offset: 30 }),
        ];

        for item in test_data.into_iter() {
            let put_res= btree_index.put(item.0, item.1);
            assert!(put_res);
        }

    }

    #[test]
    fn test_get() {
        let btree_index = BTreeIndex::new();

        let test_data=vec![
            ("test-1".as_bytes().to_vec(), LogRecordPos { file_id: 0, offset: 10 }),
            ("test-2".as_bytes().to_vec(), LogRecordPos { file_id: 0, offset: 20 }),
            ("test-3".as_bytes().to_vec(), LogRecordPos { file_id: 0, offset: 30 }),
        ];

        for item in test_data.into_iter() {
            let put_res= btree_index.put(item.0.clone(), item.1);
            assert!(put_res);
            let get_res = btree_index.get(item.0);
            assert!(get_res.is_some());
            assert_eq!(get_res.unwrap().offset, item.1.offset);
            assert_eq!(get_res.unwrap().file_id, item.1.file_id);
        }

    }

    #[test]
    fn test_delete() {
        let btree_index = BTreeIndex::new();

        let test_data=vec![
            ("test-1".as_bytes().to_vec(), LogRecordPos { file_id: 0, offset: 10 }),
            ("test-2".as_bytes().to_vec(), LogRecordPos { file_id: 0, offset: 20 }),
            ("test-3".as_bytes().to_vec(), LogRecordPos { file_id: 0, offset: 30 }),
        ];

        for item in test_data.into_iter() {
            let put_res= btree_index.put(item.0.clone(), item.1);
            assert!(put_res);
            let del_res = btree_index.delete(item.0);
            assert!(del_res);
        }

    }
}
