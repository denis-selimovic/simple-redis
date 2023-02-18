use std::collections::HashMap;
use std::sync::RwLock;

use crate::protocol::types::Type;


pub struct Storage {
    db: RwLock<HashMap<String, Type>>,
}


impl Storage {
    pub fn new() -> Self {
        Storage { db: RwLock::new(HashMap::new()) }
    }

    pub fn read(&self, key: &String) -> Option<Type> {
        let db = self.db.read().expect("Cannot obtain lock");
        
        match db.get(key) {
            None => None,
            Some(t) => Some(t.clone()),
        }
    }

    pub fn write(&mut self, key: String, t: Type) {
        let mut db = self.db.write().expect("Cannot obtain lock");
        db.insert(key, t);
    }

    pub fn remove(&mut self, key: &String) -> Type {
        let mut db = self.db.write().expect("Cannot obtain lock");
        let removed = db.remove(key);

        match removed {
            None => Type::Integer(0),
            Some(_) => Type::Integer(1),
        }
    }

    pub fn flush(&mut self) -> Type {
        let mut db = self.db.write().expect("Cannot obtain lock");
        let deleted = db.len();
        db.clear();

        Type::Integer(deleted as i64)
    }
}
