use std::io::{Cursor, Read};

use super::ParsingError;

pub struct PrimitiveParser;

impl PrimitiveParser {
    pub fn parse_i32(cursor: &mut Cursor<&[u8]>) -> Result<i32, ParsingError> {
        let mut bytes = [0; 4];

        cursor
            .read_exact(&mut bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        Ok(i32::from_le_bytes(bytes))
    }

    pub fn parse_f64(cursor: &mut Cursor<&[u8]>) -> Result<f64, ParsingError> {
        let mut bytes = [0; 8];

        cursor
            .read_exact(&mut bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        Ok(f64::from_le_bytes(bytes))
    }

    pub fn parse_bool(cursor: &mut Cursor<&[u8]>) -> Result<bool, ParsingError> {
        let mut bytes = [0; 1];

        cursor
            .read_exact(&mut bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        match bytes[0] {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(ParsingError::InvalidBool),
        }
    }

    pub fn parse_byte(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParsingError> {
        let mut byte = [0; 1];

        cursor
            .read_exact(&mut byte)
            .map_err(|_| ParsingError::Incomplete)?;

        Ok(byte[0])
    }
}
