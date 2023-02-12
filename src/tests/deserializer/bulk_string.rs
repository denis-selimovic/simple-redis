use crate::protocol::deserializer::deserialize;
use crate::protocol::types::Type;


#[test]
fn deserialize_bulk_string_empty() {
    let string = "$0\r\n\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_binary_safe() {
    let string = "$4\r\n2\r1\n\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "2\r1\n".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_null() {
    let string = "$-1\r\n\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Null => assert_eq!(1, 1),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_whitespace() {
    let string = "$12\r\nhello world!\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "hello world!".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_whitespace_capital() {
    let string = "$14\r\nHello World ! \r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "Hello World ! ".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_whitespace_only() {
    let string = "$3\r\n   \r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "   ".to_string()),
        _ => panic!("error"),
    }
}


#[test]
fn deserialize_bulk_string_numbers_only() {
    let string = "$4\r\n2345\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "2345".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_numbersl() {
    let string = "$9\r\nRust 2023\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::BulkString(s) => assert_eq!(s, "Rust 2023".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_missing_cr() {
    let string = "$14\r\nHello World ! \n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing carriage return ('\r') when parsing"#);
}

#[test]
fn deserialize_bulk_string_missing_ln() {
    let string = "$14\r\nHello World ! \r";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing line feed ('\n') when parsing"#);
}

#[test]
fn deserialize_bulk_string_missing_cr_and_ln() {
    let string = "$14\r\nHello World!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing carriage return ('\r') when parsing"#);
}

#[test]
fn deserialize_bulk_string_missing_length() {
    let string = "$\r\nHello World!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), " cannot be converted to signed 64-bit integer");
}

#[test]
fn deserialize_bulk_string_missing_prefix_length() {
    let string = "$Hello World!\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "Hello World! cannot be converted to signed 64-bit integer");
}

#[test]
fn deserialize_bulk_string_only_length() {
    let string = "$14\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "missing carriage return ('\\r') when parsing");
}

#[test]
fn deserialize_bulk_string_incompatible_length_zero() {
    let string = "$14\r\n\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "missing carriage return ('\\r') when parsing");
}

#[test]
fn deserialize_bulk_string_incompatible_length() {
    let string = "$14\r\n0123\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "missing carriage return ('\\r') when parsing");
}
