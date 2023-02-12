use crate::protocol::deserializer::deserialize;
use crate::protocol::types::Type;


#[test]
fn deserialize_simple_string_empty() {
    let string = "+\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string() {
    let string = "+hello\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "hello".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string_whitespace() {
    let string = "+hello world!\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "hello world!".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string_whitespace_capital() {
    let string = "+Hello World ! \r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "Hello World ! ".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string_whitespace_only() {
    let string = "+   \r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "   ".to_string()),
        _ => panic!("error"),
    }
}


#[test]
fn deserialize_simple_string_numbers_only() {
    let string = "+2345\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "2345".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string_numbersl() {
    let string = "+Rust 2023\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::SimpleString(s) => assert_eq!(s, "Rust 2023".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string_missing_cr() {
    let string = "+Hello World ! \n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing carriage return ('\r') when parsing"#);
}

#[test]
fn deserialize_simple_string_missing_ln() {
    let string = "+Hello World ! \r";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing line feed ('\n') when parsing"#);
}

#[test]
fn deserialize_simple_string_missing_cr_and_ln() {
    let string = "+Hello World!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing carriage return ('\r') when parsing"#);
}
