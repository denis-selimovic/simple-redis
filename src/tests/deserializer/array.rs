use crate::protocol::deserializer::deserialize;
use crate::protocol::types::Type;


#[test]
fn deserialize_empty_array() {
    let string = "*0\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => assert_eq!(arr.len(), 0),
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_integer_array() {
    let string = "*3\r\n:12\r\n:-5\r\n:2023\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 3);

            match arr[0] {
                Type::Integer(i) => assert_eq!(i, 12),
                _ => panic!("error"),
            }
            match arr[1] {
                Type::Integer(i) => assert_eq!(i, -5),
                _ => panic!("error"),
            }
            match arr[2] {
                Type::Integer(i) => assert_eq!(i, 2023),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_simple_string_array() {
    let string = "*5\r\n+hello\r\n+world\r\n+\r\n+HeLlO\r\n+1\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 5);

            match &arr[0] {
                Type::SimpleString(i) => assert_eq!(*i, "hello".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::SimpleString(i) => assert_eq!(*i, "world".to_string()),
                _ => panic!("error"),
            }
            match &arr[2] {
                Type::SimpleString(i) => assert_eq!(*i, "".to_string()),
                _ => panic!("error"),
            }
            match &arr[3] {
                Type::SimpleString(i) => assert_eq!(*i, "HeLlO".to_string()),
                _ => panic!("error"),
            }
            match &arr[4] {
                Type::SimpleString(i) => assert_eq!(*i, "1".to_string()),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_error_array() {
    let string = "*2\r\n-err1\r\n-err2\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 2);

            match &arr[0] {
                Type::Error(i) => assert_eq!(*i, "err1".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Error(i) => assert_eq!(*i, "err2".to_string()),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_bulk_string_array() {
    let string = "*4\r\n$5\r\nhello\r\n$5\r\nworld\r\n$0\r\n\r\n$5\r\nHeLlO\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 4);

            match &arr[0] {
                Type::BulkString(i) => assert_eq!(*i, "hello".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::BulkString(i) => assert_eq!(*i, "world".to_string()),
                _ => panic!("error"),
            }
            match &arr[2] {
                Type::BulkString(i) => assert_eq!(*i, "".to_string()),
                _ => panic!("error"),
            }
            match &arr[3] {
                Type::BulkString(i) => assert_eq!(*i, "HeLlO".to_string()),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_array_of_arrays() {
    let string = "*2\r\n*2\r\n-err\r\n$-1\r\n*3\r\n+hello\r\n:-54\r\n$2\r\n\r\n\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 2);

            match &arr[0] {
                Type::Array(first) => {
                    assert_eq!(first.len(), 2);
                    
                    match &first[0] {
                        Type::Error(i) => assert_eq!(*i, "err".to_string()),
                        _ => panic!("error"),
                    }
                    match &first[1] {
                        Type::Null => assert_eq!(1, 1),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Array(second) => {
                    assert_eq!(second.len(), 3);

                    match &second[0] {
                        Type::SimpleString(i) => assert_eq!(*i, "hello".to_string()),
                        _ => panic!("error"),
                    }
                    match &second[1] {
                        Type::Integer(i) => assert_eq!(*i, -54),
                        _ => panic!("error"),
                    }
                    match &second[2] {
                        Type::BulkString(i) => assert_eq!(*i, "\r\n".to_string()),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_array_of_arrays_different_types() {
    let string = "*2\r\n*2\r\n+err\r\n$-1\r\n*3\r\n-hello\r\n:-54\r\n$2\r\nhe\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 2);

            match &arr[0] {
                Type::Array(first) => {
                    assert_eq!(first.len(), 2);
                    
                    match &first[0] {
                        Type::SimpleString(i) => assert_eq!(*i, "err".to_string()),
                        _ => panic!("error"),
                    }
                    match &first[1] {
                        Type::Null => assert_eq!(1, 1),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Array(second) => {
                    assert_eq!(second.len(), 3);

                    match &second[0] {
                        Type::Error(i) => assert_eq!(*i, "hello".to_string()),
                        _ => panic!("error"),
                    }
                    match &second[1] {
                        Type::Integer(i) => assert_eq!(*i, -54),
                        _ => panic!("error"),
                    }
                    match &second[2] {
                        Type::BulkString(i) => assert_eq!(*i, "he".to_string()),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn deserialize_array_wrong_length() {
    let string = "*2\r\n*2\r\n+err\r\n$-1\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "empty byte sequence");
}

#[test]
fn deserialize_array_wrong_length_zero() {
    let string = "*1\r\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "empty byte sequence");
}

#[test]
fn deserialize_array_missing_ln() {
    let string = "*0\r";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "missing line feed ('\\n') when parsing");
}

#[test]
fn deserialize_array_missing_cr() {
    let string = "*0\n";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "missing carriage return ('\\r') when parsing");
}


#[test]
fn deserialize_array_missing_cr_lf() {
    let string = "*0";
    let buffer = string.as_bytes().to_vec();
    let mut iter = buffer.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "missing carriage return ('\\r') when parsing");
}
