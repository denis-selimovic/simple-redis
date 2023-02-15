use std::string::ToString;


#[derive(Clone)]
pub enum Type {
    Null,
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Type>),
}


impl ToString for Type {
    fn to_string(&self) -> String {
        match &self {
            Type::Null => "NULL".to_string(),
            Type::SimpleString(s) => s.to_string(),
            Type::Integer(i) => i.to_string(),
            Type::Error(e) => e.to_string(),
            Type::BulkString(s) => s.to_string(),
            Type::Array(v) => v.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" "),
        }
    }
}