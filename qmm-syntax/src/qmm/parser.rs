use std::io::Cursor;

use super::{ParsingError, Quest};

use super::{
    HeaderParser, InfoParser, JumpParser, LocationParser, ParameterParser, StringReplacementsParser,
};

pub struct QmmParser;

impl QmmParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Quest, ParsingError> {
        let header = HeaderParser::parse(cursor)?;
        let mut parameters = Vec::with_capacity(header.parameters_count);
        let mut parameters_iters = 0;

        while parameters_iters < header.parameters_count {
            parameters.push(ParameterParser::new(cursor).parse()?);

            parameters_iters += 1;
        }

        let string_replacements = StringReplacementsParser::parse(cursor)?;
        let info = InfoParser::parse(cursor)?;
        let mut locations = Vec::with_capacity(info.locations_count as usize);
        let mut locations_iter = 0;

        while locations_iter < info.locations_count {
            locations.push(LocationParser::parse(cursor)?);
            locations_iter += 1;
        }

        let mut jumps = Vec::with_capacity(info.jumps_count as usize);
        let mut jumps_iter = 0;

        while jumps_iter < info.jumps_count {
            jumps.push(JumpParser::parse(cursor)?);
            jumps_iter += 1;
        }

        if (cursor.position() as usize) != cursor.get_ref().len() {
            return Err(ParsingError::ExpectedEnd);
        }

        Ok(Quest {
            header,
            parameters,
            string_replacements,
            info,
            locations,
            jumps,
        })
    }
}
