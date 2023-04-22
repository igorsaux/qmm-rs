use std::io::Cursor;

use super::{ParsingError, StringParser, StringReplacements};

pub struct StringReplacementsParser;

impl StringReplacementsParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<StringReplacements, ParsingError> {
        let to_star = StringParser::parse(cursor)?;
        let to_planet = StringParser::parse(cursor)?;

        // Skip '<Date>' and '<Money>' strings
        StringParser::parse(cursor)?;
        StringParser::parse(cursor)?;

        let from_planet = StringParser::parse(cursor)?;
        let from_star = StringParser::parse(cursor)?;
        let ranger = StringParser::parse(cursor)?;

        Ok(StringReplacements {
            to_star,
            to_planet,
            from_planet,
            from_star,
            ranger,
        })
    }
}
