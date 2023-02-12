use crate::protocol::deserializer::deserialize;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;


#[test]
fn serialize_null() {
    let bytes = serialize(&Type::Null);
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Null => assert_eq!(1, 1),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_simple_string() {
    let bytes = serialize(&Type::SimpleString("hello".to_string()));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::SimpleString(s)=> assert_eq!(s, "hello".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_error() {
    let bytes = serialize(&Type::Error("ERR: Code 1".to_string()));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Error(s)=> assert_eq!(s, "ERR: Code 1".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_positive_integer() {
    let bytes = serialize(&Type::Integer(123));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Integer(i)=> assert_eq!(i, 123),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_negative_integer() {
    let bytes = serialize(&Type::Integer(-4012));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Integer(i)=> assert_eq!(i, -4012),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_bulk_string() {
    let bytes = serialize(&Type::BulkString("hello world!".to_string()));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::BulkString(s)=> assert_eq!(s, "hello world!".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_bulk_string_binary_safe() {
    let bytes = serialize(&Type::BulkString("hello\r\nworld!".to_string()));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::BulkString(s)=> assert_eq!(s, "hello\r\nworld!".to_string()),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_empty_array() {
    let bytes = serialize(&Type::Array(vec![]));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => assert_eq!(arr.len(), 0),
        _ => panic!("error"),
    }
}

#[test]
fn serialize_array_of_nulls() {
    let arr = vec![Type::Null, Type:: Null];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 2);

            match &arr[0] {
                Type::Null => assert_eq!(1, 1),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Null => assert_eq!(1, 1),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn serialize_array_of_simple_strings() {
    let arr = vec![Type::SimpleString("h".to_string()), Type::SimpleString("w".to_string())];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 2);

            match &arr[0] {
                Type::SimpleString(i) => assert_eq!(*i, "h".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::SimpleString(i) => assert_eq!(*i, "w".to_string()),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}


#[test]
fn serialize_array_of_errors() {
    let arr = vec![Type::Error("err".to_string())];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 1);

            match &arr[0] {
                Type::Error(i) => assert_eq!(*i, "err".to_string()),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn serialize_array_of_integers() {
    let arr = vec![Type::Integer(1), Type::Integer(-54), Type::Integer(3)];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 3);

            match &arr[0] {
                Type::Integer(i) => assert_eq!(*i, 1),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Integer(i) => assert_eq!(*i, -54),
                _ => panic!("error"),
            }
            match &arr[2] {
                Type::Integer(i) => assert_eq!(*i, 3),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn serialize_array_of_bulk_strings() {
    let arr = vec![
        Type::BulkString("/r\n\rHello World !!".to_string()),
        Type::Null,
        Type::BulkString("hello 2023".to_string()),
    ];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 3);

            match &arr[0] {
                Type::BulkString(i) => assert_eq!(*i, "/r\n\rHello World !!".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Null => assert_eq!(1, 1),
                _ => panic!("error"),
            }
            match &arr[2] {
                Type::BulkString(i) => assert_eq!(*i, "hello 2023".to_string()),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn serialize_array_of_arrays() {
    let arr = vec![
        Type::BulkString("/r\n\rHello World !!".to_string()),
        Type::Array(
            vec![
                Type::Null,
                Type::Integer(-2),
            ]
        ),
        Type::Error("err".to_string()),
        Type::Array(
            vec![
                Type::SimpleString("hello world".to_string())
            ]
        ),
    ];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 4);

            match &arr[0] {
                Type::BulkString(i) => assert_eq!(*i, "/r\n\rHello World !!".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Array(arr) => {
                    assert_eq!(arr.len(), 2);
                },
                _ => panic!("error"),
            }
            match &arr[2] {
                Type::Error(i) => assert_eq!(*i, "err".to_string()),
                _ => panic!("error"),
            }
            match &arr[3] {
                Type::Array(arr) => {
                    assert_eq!(arr.len(), 1);
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn serialize_array_of_arrays_check_elements() {
    let arr = vec![
        Type::BulkString("/r\n\rHello World !!".to_string()),
        Type::Array(
            vec![
                Type::Null,
                Type::Integer(-2),
            ]
        ),
        Type::Error("err".to_string()),
        Type::Array(
            vec![
                Type::SimpleString("hello world".to_string())
            ]
        ),
    ];
    let bytes = serialize(&Type::Array(arr));
    let mut iter = bytes.into_iter();

    let res = deserialize(&mut iter);
    assert_eq!(res.is_ok(), true);

    let t = res.ok().unwrap();
    match t {
        Type::Array(arr) => {
            assert_eq!(arr.len(), 4);

            match &arr[0] {
                Type::BulkString(i) => assert_eq!(*i, "/r\n\rHello World !!".to_string()),
                _ => panic!("error"),
            }
            match &arr[1] {
                Type::Array(arr) => {
                    assert_eq!(arr.len(), 2);

                    match &arr[0] {
                        Type::Null => assert_eq!(1, 1),
                        _ => panic!("error"),
                    }
                    match &arr[1] {
                        Type::Integer(i) => assert_eq!(*i, -2),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &arr[2] {
                Type::Error(i) => assert_eq!(*i, "err".to_string()),
                _ => panic!("error"),
            }
            match &arr[3] {
                Type::Array(arr) => {
                    assert_eq!(arr.len(), 1);

                    match &arr[0] {
                        Type::SimpleString(s) => assert_eq!(*s, "hello world".to_string()),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}
