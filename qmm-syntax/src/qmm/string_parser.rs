use std::io::{Cursor, Read};

use super::ParsingError;

pub struct StringParser;

impl StringParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<String, ParsingError> {
        let mut has_string_bytes = [0; 4];

        cursor
            .read_exact(&mut has_string_bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        let has_string = u32::from_le_bytes(has_string_bytes);

        if has_string == 0x00 {
            return Ok(String::new());
        }

        let mut string_length_bytes = [0; 4];

        cursor
            .read_exact(&mut string_length_bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        let string_length = u32::from_le_bytes(string_length_bytes) as usize * 2;

        if string_length == 0 {
            return Ok(String::new());
        }

        let mut string_bytes = vec![0; string_length];

        cursor
            .read_exact(&mut string_bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        let name_bytes =
            bytemuck::try_cast_slice(&string_bytes).map_err(|_| ParsingError::InvalidString)?;

        String::from_utf16(name_bytes).map_err(|_| ParsingError::InvalidString)
    }
}
