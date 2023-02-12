use std::iter::IntoIterator;
use std::result;
use crate::errors::parse::ParsingError;
use crate::protocol::types::Type;


pub type ParsingResult = result::Result<Type, ParsingError>;


pub fn deserialize<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    let mut iter = buffer.into_iter();

    match iter.next() {
        None => Err(ParsingError::Empty),
        Some(start_byte) => handler(iter, start_byte),
    }
}

fn handler<T>(buffer: T, start_byte: u8) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    match start_byte {
        43 => simple_string(buffer),
        45 => error(buffer),
        58 => integer(buffer),
        36 => bulk_string(buffer),
        42 => array(buffer),
        _ => Err(ParsingError::UnknownStartByte(start_byte)),
    }
}

fn simple_string<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    Ok(Type::Null)
}

fn error<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    Ok(Type::Null)
}

fn integer<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    Ok(Type::Null)
}

fn bulk_string<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    Ok(Type::Null)
}

fn array<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    Ok(Type::Null)
}
