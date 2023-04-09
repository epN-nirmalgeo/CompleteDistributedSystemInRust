use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyValueMap {
    memory_map: HashMap<String, String>,
}

impl KeyValueMap {
    pub fn new() -> KeyValueMap {
        KeyValueMap {
            memory_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.memory_map.insert(key, value);
    }

    pub fn get(&self, key: String) {
        self.memory_map.get(&key);
    }

    pub fn remove(&mut self, key: String) {
        self.memory_map.remove(&key);
    }
}
