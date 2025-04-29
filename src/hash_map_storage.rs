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

    fn remove(&mut self, key: &str) -> Option<Record> {
        self.0.remove(key)
    }

    fn find(&mut self, key: &str) -> Option<&Record> {
        let expired = self
            .0
            .get(key)
            .map(|record| record.is_expired())
            .unwrap_or(false);

        if expired {
            self.remove(key);
            None
        } else {
            self.0.get(key)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage;

    use super::*;

    #[test]
    fn test_storage() {
        let mut storage = HashMapStorage::new();

        let key = "test".to_owned();
        let value = b"hello".to_vec();
        let mut record = Record::default();
        record.data = value.clone();

        storage.store(key.to_owned(), record.clone());
        assert_eq!(record, *storage.find(&key).unwrap());
        assert_eq!(record.data, value);

        assert_eq!(None, storage.find("non-existing"));
    }

    #[test]
    fn test_never_expiring_key() {
        let mut storage = HashMapStorage::new();

        let key = "test";
        let value = b"hello".to_vec();
        let mut record = Record::new_with_expire_time(storage::NEVER_EXPIRES);

        record.data = value.clone();
        storage.store(key.to_owned(), record.clone());
        assert_eq!(record, *storage.find(&key).unwrap());
        assert_eq!(record.data, value);
    }

    #[test]
    fn test_immediately_expiring_key() {
        let mut storage = HashMapStorage::new();

        let key = "test";
        let value = b"hello".to_vec();
        let mut record = Record::new_with_expire_time(storage::IMMEDIATELY_EXPIRES);

        record.data = value.clone();
        storage.store(key.to_owned(), record.clone());
        assert_eq!(None, storage.find(&key));
    }

    #[test]
    fn test_with_expired_key() {
        let mut storage = HashMapStorage::new();

        let key = "test";
        let value = b"hello".to_vec();
        let mut record = Record::new_with_expire_time(12345);

        record.data = value.clone();
        storage.store(key.to_owned(), record.clone());
        assert_eq!(None, storage.find(&key));
    }
}
