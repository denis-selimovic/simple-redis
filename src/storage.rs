use std::collections::HashMap;
use crate::protocol::types::Type;


pub struct Storage {
    db: HashMap<String, Type>,
}


impl Storage {
    pub fn new() -> Self {
        Storage { db: HashMap::new() }
    }

    pub fn read(&self, key: &String) -> Option<&Type> {
        self.db.get(key)
    }

    pub fn write(&mut self, key: String, t: Type) {
        self.db.insert(key, t);
    }
}
