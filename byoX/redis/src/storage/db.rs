use crate::storage::value::Value;
use std::collections::HashMap;

pub struct Db {
    data: HashMap<String, Value>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.data.get(key).map(|Value::String(bytes)| bytes.clone())
    }

    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, Value::String(value));
    }

    pub fn del(&mut self, keys: &[String]) -> i64 {
        let mut count = 0;
        for key in keys {
            if self.data.remove(key).is_some() {
                count += 1;
            }
        }

        count
    }
}
