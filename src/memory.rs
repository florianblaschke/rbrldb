use std::collections::HashMap;

pub trait Store {
    fn new() -> Self;
    fn set(&mut self, key: String, value: String);
    fn get(&self, key: String) -> Option<String>;
}

pub struct Db {
    map: HashMap<String, String>,
}

impl Store for Db {
    fn new() -> Self {
        Db {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }
}
