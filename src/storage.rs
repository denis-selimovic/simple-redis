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

    pub fn remove(&mut self, key: &String) -> Type {
        let removed = self.db.remove(key);

        match removed {
            None => Type::Integer(0),
            Some(_) => Type::Integer(1),
        }
    }

    pub fn flush(&mut self) -> Type {
        let deleted = self.db.len();
        self.db.clear();

        Type::Integer(deleted as i64)
    }
}
