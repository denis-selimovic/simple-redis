use crate::errors::parse::ParsingError;
use crate::protocol::types::Type;


pub type ParsingResult = Result<Type, ParsingError>;


pub fn deserialize<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    let next = buffer.next();
    let parsed = handler(buffer, next)?;

    match buffer.next() {
        None => return Err(ParsingError::MissingEndByte),
        Some(byte) => {
            if byte != 33 {
                return Err(ParsingError::MissingEndByte);
            }
        }
    }

    Ok(parsed)
}

fn handler<T>(buffer: &mut T, start_byte: Option<u8>) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    match start_byte {
        Some(start_byte) => match start_byte {
            43 => simple_string(buffer),
            45 => error(buffer),
            58 => integer(buffer),
            36 => bulk_string(buffer),
            42 => array(buffer),
            _ => Err(ParsingError::UnknownStartByte(start_byte)),
        },
        None => Err(ParsingError::Empty),
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
    let integer_string = extract_bytes(buffer)?;
    let parsing_result = integer_string.parse::<i64>();

    match parsing_result {
        Err(_) => Err(ParsingError::IntegerOverflow(integer_string)),
        Ok(integer) => Ok(Type::Integer(integer)),
    }
}

fn bulk_string<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    let len = extract_bytes(buffer)?;
    let parsed_len = len.parse::<i64>();

    match parsed_len {
        Err(_) => Err(ParsingError::IntegerOverflow(len)),
        Ok(integer) => {
            if integer < 0 {
                if integer == -1 {
                    let _ = extract_bulk_bytes(buffer, 0);
                    return Ok(Type::Null);
                } else {
                    return Err(ParsingError::InvalidStringLength(integer));
                }
            } else {
                let integer = integer as usize;
                let string = extract_bulk_bytes(buffer, integer)?;

                return Ok(Type::BulkString(string));
            }
        },
    }
}

fn array<T>(buffer: &mut T) -> ParsingResult
where
    T: Iterator<Item = u8>
{
    let len = extract_bytes(buffer)?;
    let parsed_len = len.parse::<u64>();

    match parsed_len {
        Err(_) => Err(ParsingError::InvalidArrayLength),
        Ok(len) => {
            let mut array = vec![];
            
            for _ in 0..len {
                let next = buffer.next();
                let el = handler(buffer, next)?;
                array.push(el);
            }

            Ok(Type::Array(array))
        }
    }
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

fn extract_bulk_bytes<T>(buffer: &mut T, size: usize) -> Result<String, ParsingError>
where
    T: Iterator<Item = u8>
{
    let mut string = String::new();
    let mut encounter_cr = false;
    let mut encounter_lf = false;

    for _ in 0..size {
        if let Some(byte) = buffer.next() {
            string.push(byte as char);
        }
    }

    if let Some(byte) = buffer.next() {
        if byte == 13 {
            encounter_cr = true;
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

    if string.len() < size {
        return Err(ParsingError::StringLengthMismatch(string.len(), size));
    }

    Ok(string)
}