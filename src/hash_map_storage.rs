use crate::storage::{Record, Storage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HashMapStorage(HashMap<String, Record>);

impl HashMapStorage {
    pub fn new() -> HashMapStorage {
        HashMapStorage(HashMap::new())
    }
}

impl Storage for HashMapStorage {
    fn store(&mut self, key: String, value: Record) {
        self.0.insert(key, value);
    }

    fn find(&self, key: &String) -> Option<&Record> {
        self.0.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let mut storage = HashMapStorage::new();

        let key = "test".to_owned();
        let value = b"hello".to_vec();
        let mut record = Record::default();
        record.data = value.clone();

        storage.store(key.clone(), record.clone());
        assert_eq!(record, *storage.find(&key).unwrap());
        assert_eq!(record.data, value);

        assert_eq!(None, storage.find(&"non-existing".to_owned()));
    }
}
