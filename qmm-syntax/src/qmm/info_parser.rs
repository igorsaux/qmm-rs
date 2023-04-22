use std::io::Cursor;

use crate::text::formatted_text::FormattedText;

use super::{Info, ParsingError, PrimitiveParser, StringParser};

pub struct InfoParser;

impl InfoParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Info, ParsingError> {
        let locations_count = PrimitiveParser::parse_i32(cursor)? as u32;
        let jumps_count = PrimitiveParser::parse_i32(cursor)? as u32;
        let success_text = FormattedText::parse(&StringParser::parse(cursor)?);
        let task_text = FormattedText::parse(&StringParser::parse(cursor)?);

        Ok(Info {
            locations_count,
            jumps_count,
            success_text,
            task_text,
        })
    }
}
