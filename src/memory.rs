use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Store {
    fn new() -> Self;
    fn set(&mut self, key: String, value: Value);
    fn get(&self, key: String) -> Vec<u8>;
}

#[derive(Debug)]
pub struct Db {
    map: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Value {
    pub data: Vec<u8>,
    pub ttl: Option<i64>,
}

impl Store for Db {
    fn new() -> Self {
        Db {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: Value) {
        self.map.insert(key, value);
    }

    fn get(&self, key: String) -> Vec<u8> {
        let value = self.map.get(&key);
        let res = match value {
            Some(v) => v.data.clone(),
            None => Vec::new(),
        };

        res
    }
}
