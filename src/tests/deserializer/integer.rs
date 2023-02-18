use crate::protocol::deserializer::deserialize;
use crate::protocol::types::Type;


#[test]
fn deserialize_integer_zero() {
    let string = ":0\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, 0),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_positive() {
    let string = ":2023\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, 2023),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_negative() {
    let string = ":-2023\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, -2023),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_multiple_digits() {
    let string = ":123456789012345\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, 123456789012345),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_multiple_digits_negative() {
    let string = ":-123456789012345\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, -123456789012345),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_upper_bound() {
    let string = ":9223372036854775807\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, 9223372036854775807),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_lower_bound() {
    let string = ":-9223372036854775808\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, -9223372036854775808),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_overflow() {
    let string = ":18446744073709551616\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "18446744073709551616 cannot be converted to signed 64-bit integer");
}

#[test]
fn deserialize_integer_overflow_negative() {
    let string = ":-18446744073709551617\r\n!";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "-18446744073709551617 cannot be converted to signed 64-bit integer");
}

#[test]
fn deserialize_integer_missing_cr() {
    let string = ":123\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing carriage return ('\r') when parsing"#);
}

#[test]
fn deserialize_integer_missing_ln() {
    let string = ":444\r";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing line feed ('\n') when parsing"#);
}

#[test]
fn deserialize_integer_missing_cr_and_ln() {
    let string = ":2022";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), r#"missing carriage return ('\r') when parsing"#);
}
