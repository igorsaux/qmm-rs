use std::io::Cursor;

use crate::text::formula::Formula;

use super::{
    MediaParser, ParameterChange, ParameterChangeError, ParameterChangeType, ParameterShowType,
    ParsingError, PrimitiveParser, StringParser,
};

pub struct ParameterChangeParser;

impl ParameterChangeParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<ParameterChange, ParsingError> {
        let parameter_id = PrimitiveParser::parse_i32(cursor)? as u32;
        PrimitiveParser::parse_i32(cursor)?;

        let show_type =
            ParameterShowType::try_from(PrimitiveParser::parse_byte(cursor)?).map_err(|_| {
                ParsingError::InvalidParameterChange(ParameterChangeError::InvalidShowType)
            })?;
        let change_type = ParameterChangeType::try_from(PrimitiveParser::parse_byte(cursor)?)
            .map_err(|_| {
                ParsingError::InvalidParameterChange(ParameterChangeError::InvalidChangeType)
            })?;
        let formula_text = StringParser::parse(cursor)?;
        let formula =
            Formula::parse(&formula_text).map_err(|err| ParsingError::InvalidFormula {
                formula: formula_text,
                error: err,
            })?;
        let critical_text = StringParser::parse(cursor)?;
        let media = MediaParser::parse(cursor)?;

        Ok(ParameterChange {
            parameter_id,
            show_type,
            change_type,
            formula,
            critical_text,
            media,
        })
    }
}
