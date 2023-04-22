use std::io::Cursor;

use crate::text::{formatted_text::FormattedText, formula::Formula};

use super::{
    Jump, JumpId, JumpParameterConditionParser, LocationId, MaxVisits, MediaParser,
    ParameterChangeParser, ParsingError, PrimitiveParser, StringParser,
};

pub struct JumpParser;

impl JumpParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Jump, ParsingError> {
        let priority = PrimitiveParser::parse_f64(cursor)?;
        let do_pass_day = PrimitiveParser::parse_i32(cursor)? > 0;
        let id = JumpId(PrimitiveParser::parse_i32(cursor)? as u32);
        let from = LocationId(PrimitiveParser::parse_i32(cursor)? as u32);
        let to = LocationId(PrimitiveParser::parse_i32(cursor)? as u32);
        let show_always = PrimitiveParser::parse_bool(cursor)?;
        let max_visits = PrimitiveParser::parse_i32(cursor)? as u32;
        let max_visits = match max_visits {
            0 => MaxVisits::Infinite,
            _ => MaxVisits::Limit(max_visits),
        };
        let show_order = PrimitiveParser::parse_i32(cursor)? as u32;
        let jump_parameters_conditions_count = PrimitiveParser::parse_i32(cursor)?;
        let mut parameters_conditions =
            Vec::with_capacity(jump_parameters_conditions_count as usize);
        let mut jump_parameters_conditions_iter = 0;

        while jump_parameters_conditions_iter < jump_parameters_conditions_count {
            parameters_conditions.push(JumpParameterConditionParser::parse(cursor)?);

            jump_parameters_conditions_iter += 1;
        }

        let parameters_changes_count = PrimitiveParser::parse_i32(cursor)?;
        let mut parameter_changes = Vec::with_capacity(parameters_changes_count as usize);
        let mut parameters_changes_iter = 0;

        while parameters_changes_iter < parameters_changes_count {
            parameter_changes.push(ParameterChangeParser::parse(cursor)?);

            parameters_changes_iter += 1;
        }

        let formula_text = StringParser::parse(cursor)?;
        let formula =
            Formula::parse(&formula_text).map_err(|err| ParsingError::InvalidFormula {
                error: err,
                formula: formula_text,
            })?;
        let text = FormattedText::parse(&StringParser::parse(cursor)?);
        let description = FormattedText::parse(&StringParser::parse(cursor)?);
        let media = MediaParser::parse(cursor)?;

        Ok(Jump {
            priority,
            do_pass_day,
            id,
            to,
            from,
            show_always,
            max_visits,
            show_order,
            parameters_conditions,
            parameter_changes,
            formula,
            text,
            description,
            media,
        })
    }
}
