use std::io::Cursor;

use super::{JumpParameterCondition, ParsingError, PrimitiveParser};

pub struct JumpParameterConditionParser;

impl JumpParameterConditionParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<JumpParameterCondition, ParsingError> {
        let parameter_id = PrimitiveParser::parse_i32(cursor)? as u32;
        let range_start = PrimitiveParser::parse_i32(cursor)?;
        let range_end = PrimitiveParser::parse_i32(cursor)?;
        let must_equal_values_count = PrimitiveParser::parse_i32(cursor)?;
        let must_equal = PrimitiveParser::parse_bool(cursor)?;
        let mut must_equal_values = Vec::with_capacity(must_equal_values_count as usize);
        let mut must_equal_values_iter = 0;

        while must_equal_values_iter < must_equal_values_count {
            must_equal_values.push(PrimitiveParser::parse_i32(cursor)?);
            must_equal_values_iter += 1;
        }

        let must_mod_values_count = PrimitiveParser::parse_i32(cursor)?;
        let must_mod = PrimitiveParser::parse_bool(cursor)?;
        let mut must_mod_values = Vec::with_capacity(must_mod_values_count as usize);
        let mut must_mod_values_iter = 0;

        while must_mod_values_iter < must_mod_values_count {
            must_mod_values.push(PrimitiveParser::parse_i32(cursor)?);
            must_mod_values_iter += 1;
        }

        Ok(JumpParameterCondition {
            parameter_id,
            range_start,
            range_end,
            must_equal,
            must_equal_values,
            must_mod,
            must_mod_values,
        })
    }
}
