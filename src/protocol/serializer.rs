use crate::protocol::types::Type;


pub fn serialize(t: &Type) -> Vec<u8> {
    match t {
        Type::Null => null(),
        Type::SimpleString(s) => simple_string(&s),
        Type::Error(s) => error(&s),
        Type::Integer(i) => integer(*i),
        Type::BulkString(s) => bulk_string(&s),
        Type::Array(arr) => array(&arr),
    }
}

fn null() -> Vec<u8> {
    vec![36, 45, 49, 13, 10]
}

fn simple_string(s: &str) -> Vec<u8> {
    let mut v = vec![43];
    extract_ascii(s, &mut v);

    v.push(13);
    v.push(10);

    v
}

fn error(s: &str) -> Vec<u8> {
    let mut v = vec![45];
    extract_ascii(s, &mut v);

    v.push(13);
    v.push(10);

    v
}

fn integer(i: i64) -> Vec<u8> {
    let mut v = vec![58];
    extract_ascii(&i.to_string(), &mut v);

    v.push(13);
    v.push(10);

    v
}

fn bulk_string(s: &str) -> Vec<u8> {
    let mut v = vec![36];
    extract_ascii(&s.len().to_string(), &mut v);
    v.push(13);
    v.push(10);

    extract_ascii(s, &mut v);
    v.push(13);
    v.push(10);

    v
}

fn array(arr: &Vec<Type>) -> Vec<u8> {
    let mut v = vec![42];
    extract_ascii(&arr.len().to_string(), &mut v);
    v.push(13);
    v.push(10);

    for el in arr {
        let mut code = serialize(el);
        v.append(&mut code);
    }

    v
}

fn extract_ascii(s: &str, v: &mut Vec<u8>) {
    for ch in s.bytes() {
        v.push(ch);
    }
}
