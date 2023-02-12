use std::iter::IntoIterator;
use super::types::Type;

pub fn deserialize<T>(buffer: T) -> Type
where
    T: IntoIterator<Item = u8>
{
    let mut iter = buffer.into_iter();

    match iter.next() {
        None => Type::Null,
        Some(start_byte) => handler(iter, start_byte),
    }
}

fn handler<T>(buffer: T, start_byte: u8) -> Type
where
    T: IntoIterator<Item = u8>
{
    match start_byte {
        43 => simple_string(buffer),
        45 => error(buffer),
        58 => integer(buffer),
        36 => bulk_string(buffer),
        42 => array(buffer),
        _ => Type::Null
    }
}

fn simple_string<T>(buffer: T) -> Type
where
    T: IntoIterator<Item = u8>
{
    Type::Null
}

fn error<T>(buffer: T) -> Type
where
    T: IntoIterator<Item = u8>
{
    Type::Null
}

fn integer<T>(buffer: T) -> Type
where
    T: IntoIterator<Item = u8>
{
    Type::Null
}

fn bulk_string<T>(buffer: T) -> Type
where
    T: IntoIterator<Item = u8>
{
    Type::Null
}

fn array<T>(buffer: T) -> Type
where
    T: IntoIterator<Item = u8>
{
    Type::Null
}
