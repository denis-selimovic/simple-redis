#[derive(Clone)]
pub enum Type {
    Null,
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Type>),
}

