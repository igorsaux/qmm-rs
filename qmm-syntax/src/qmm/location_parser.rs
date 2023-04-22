use std::io::Cursor;

use crate::text::{formatted_text::FormattedText, formula::Formula};

use super::{
    Location, LocationError, LocationId, LocationSelectType, LocationType, MaxVisits, MediaParser,
    ParameterChangeParser, ParsingError, PrimitiveParser, StringParser,
};

pub struct LocationParser;

impl LocationParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Location, ParsingError> {
        let do_pass_day = PrimitiveParser::parse_i32(cursor)? > 0;

        // Skip coordinates
        PrimitiveParser::parse_i32(cursor)?;
        PrimitiveParser::parse_i32(cursor)?;

        let id = LocationId(PrimitiveParser::parse_i32(cursor)? as u32);
        let max_visits = PrimitiveParser::parse_i32(cursor)? as u32;
        let max_visits = match max_visits {
            0 => MaxVisits::Infinite,
            _ => MaxVisits::Limit(max_visits),
        };
        let ty = LocationType::try_from(PrimitiveParser::parse_byte(cursor)?)
            .map_err(|_| ParsingError::InvalidLocation(LocationError::InvalidLocationType))?;

        let parameters_changes_count = PrimitiveParser::parse_i32(cursor)?;
        let mut parameter_changes = Vec::with_capacity(parameters_changes_count as usize);
        let mut parameters_changes_iter = 0;

        while parameters_changes_iter < parameters_changes_count {
            parameter_changes.push(ParameterChangeParser::parse(cursor)?);

            parameters_changes_iter += 1;
        }

        let location_texts_count = PrimitiveParser::parse_i32(cursor)?;
        let mut texts = Vec::with_capacity(location_texts_count as usize);
        let mut media = Vec::with_capacity(location_texts_count as usize);
        let mut location_texts_iter = 0;

        while location_texts_iter < location_texts_count {
            texts.push(FormattedText::parse(&StringParser::parse(cursor)?));
            media.push(MediaParser::parse(cursor)?);

            location_texts_iter += 1;
        }

        let select_type = PrimitiveParser::parse_bool(cursor)?;
        let select_formula = StringParser::parse(cursor)?;
        let select_type = match select_type {
            false => LocationSelectType::ByOrder,
            true => {
                let formula = Formula::parse(&select_formula).map_err(|err| {
                    ParsingError::InvalidFormula {
                        error: err,
                        formula: select_formula,
                    }
                })?;
                LocationSelectType::ByFormula(formula)
            }
        };

        Ok(Location {
            do_pass_day,
            id,
            max_visits,
            ty,
            parameter_changes,
            texts,
            media,
            select_type,
        })
    }
}
