use crate::data::log_record::LogRecordPos;
use crate::index::{Index, IndexIterator};
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;
use bytes::Bytes;
use crate::options::IteratorOptions;

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

impl Index for BTreeIndex {
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

    fn list_keys(&self) -> Option<Vec<Bytes>> {
        let read_guard=self.index.read();
        let mut keys=Vec::with_capacity(read_guard.len());
        for (item,_) in read_guard.iter() {
            keys.push(Bytes::copy_from_slice(item));
        }
        Some(keys)
    }

    fn iterator(&self, options: IteratorOptions) -> Box<dyn IndexIterator> {
        let read_guard=self.index.read();
        let mut items=Vec::with_capacity(read_guard.len());
        for (key, value) in read_guard.iter() {
            items.push((key.clone(),value.clone()));
        }
        if options.reverse {
            items.reverse();
        }
        Box::new(BTreeIndexIterator{
            items,
            index: 0,
            options,
        })
    }
}

pub struct BTreeIndexIterator{
    items:Vec<(Vec<u8>,LogRecordPos)>,
    index:usize,
    options:IteratorOptions,
}

impl IndexIterator for BTreeIndexIterator {
    fn rewind(&mut self) {
        self.index=0;
    }

    fn seek(&mut self, key: Vec<u8>) {

    }

    fn next(&mut self) -> Option<(Vec<u8>, LogRecordPos)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::log_record::LogRecordPos;
    use crate::index::btree::BTreeIndex;
    use crate::index::Index;

    #[test]
    fn test_put() {
        let btree_index = BTreeIndex::new();

        let put_res1 = btree_index.put(
            "test-1".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 0,
                offset: 10,
            },
        );
        assert!(put_res1);

        let put_res2 = btree_index.put(
            "test-2".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 0,
                offset: 20,
            },
        );
        assert!(put_res2);

        let put_res3 = btree_index.put(
            "test-3".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 0,
                offset: 30,
            },
        );
        assert!(put_res3);
    }

    #[test]
    fn test_get() {
        let btree_index = BTreeIndex::new();

        let put_res1 = btree_index.put(
            "test-1".into(),
            LogRecordPos {
                file_id: 0,
                offset: 10,
            },
        );
        assert!(put_res1);

        let put_res2 = btree_index.put(
            "test-2".into(),
            LogRecordPos {
                file_id: 0,
                offset: 20,
            },
        );
        assert!(put_res2);

        let put_res3 = btree_index.put(
            "test-3".into(),
            LogRecordPos {
                file_id: 0,
                offset: 30,
            },
        );
        assert!(put_res3);

        let get_res1 = btree_index.get("test-1".into());
        assert!(get_res1.is_some());
        assert_eq!(get_res1.unwrap().offset, 10);
        assert_eq!(get_res1.unwrap().file_id, 0);

        let get_res2 = btree_index.get("test-2".into());
        assert!(get_res2.is_some());
        assert_eq!(get_res2.unwrap().offset, 20);
        assert_eq!(get_res2.unwrap().file_id, 0);

        let get_res3 = btree_index.get("test-3".into());
        assert!(get_res3.is_some());
        assert_eq!(get_res3.unwrap().offset, 30);
        assert_eq!(get_res3.unwrap().file_id, 0);
    }

    #[test]
    fn test_delete() {
        let btree_index = BTreeIndex::new();

        let put_res1 = btree_index.put(
            "test-1".into(),
            LogRecordPos {
                file_id: 0,
                offset: 10,
            },
        );
        assert!(put_res1);

        let put_res2 = btree_index.put(
            "test-2".into(),
            LogRecordPos {
                file_id: 0,
                offset: 20,
            },
        );
        assert!(put_res2);

        let put_res3 = btree_index.put(
            "test-3".into(),
            LogRecordPos {
                file_id: 0,
                offset: 30,
            },
        );
        assert!(put_res3);

        let del_res1 = btree_index.delete("test-1".into());
        assert!(del_res1);
        let del_res2 = btree_index.delete("test-2".into());
        assert!(del_res2);
        let del_res2 = btree_index.delete("test-3".into());
        assert!(del_res2);
    }


}
