use std::iter::IntoIterator;
use crate::errors::parse::ParsingError;
use crate::protocol::types::Type;


pub type ParsingResult = Result<Type, ParsingError>;


pub fn deserialize<T>(buffer: T) -> ParsingResult
where
    T: IntoIterator<Item = u8>
{
    let mut iter = buffer.into_iter();

    match iter.next() {
        None => Err(ParsingError::Empty),
        Some(start_byte) => handler(&mut iter, start_byte),
    }
}

fn handler<T>(buffer: &mut T, start_byte: u8) -> ParsingResult
where
    T: Iterator<Item = u8>
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

fn simple_string<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    let simple_string = extract_bytes(buffer)?;
    Ok(Type::SimpleString(simple_string))
}

fn error<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    let error = extract_bytes(buffer)?;
    Ok(Type::Error(error))
}

fn integer<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    Ok(Type::Null)
}

fn bulk_string<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    Ok(Type::Null)
}

fn array<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    Ok(Type::Null)
}

fn extract_bytes<T>(buffer: &mut T) -> Result<String, ParsingError>
where
    T: Iterator<Item = u8>
{
    let mut string = String::new();
    let mut encounter_cr = false;
    let mut encounter_lf = false;

    while let Some(byte) = buffer.next() {
        match byte {
            13 => {
                encounter_cr = true;
                break;
            },
            _ => string.push(byte as char),
        }
    }

    if let Some(byte) = buffer.next() {
        if byte == 10 {
            encounter_lf = true;
        }
    }

    if !encounter_cr {
        return Err(ParsingError::MissingCR);
    }
    if !encounter_lf {
        return Err(ParsingError::MissingLF);
    }

    Ok(string)
}