use std::io::{Cursor, Seek};

use super::{
    CriticalValue, FormattedRangeLine, Parameter, ParameterError, ParameterType, ParsingError,
    PrimitiveParser, StringParser,
};

pub struct ParameterParser<'c, 'd> {
    cursor: &'c mut Cursor<&'d [u8]>,
}

impl<'c, 'd> ParameterParser<'c, 'd> {
    pub fn new(cursor: &'c mut Cursor<&'d [u8]>) -> Self {
        Self { cursor }
    }

    pub fn parse(&mut self) -> Result<Parameter, ParsingError> {
        let min_value = PrimitiveParser::parse_i32(self.cursor)?;
        let max_value = PrimitiveParser::parse_i32(self.cursor)?;
        let ty = self.parse_type()?;

        // Skip zero bytes
        self.cursor
            .seek(std::io::SeekFrom::Current(3))
            .map_err(|_| ParsingError::Incomplete)?;

        let show_when_zero = PrimitiveParser::parse_bool(self.cursor)?;
        let critical_value = self.parse_critical_value()?;
        let is_active = PrimitiveParser::parse_bool(self.cursor)?;
        let formatted_lines_count = PrimitiveParser::parse_i32(self.cursor)? as usize;
        let is_money = PrimitiveParser::parse_bool(self.cursor)?;
        let name = StringParser::parse(self.cursor)?;
        let formatted_range_lines = self.parse_formatted_range_lines(formatted_lines_count)?;
        let critical_text = StringParser::parse(self.cursor)?;
        let image = StringParser::parse(self.cursor)?;
        let sound = StringParser::parse(self.cursor)?;
        let track = StringParser::parse(self.cursor)?;
        let starting_value = StringParser::parse(self.cursor)?;

        Ok(Parameter {
            min_value,
            max_value,
            ty,
            show_when_zero,
            critical_value,
            is_active,
            is_money,
            name,
            formatted_range_lines,
            critical_text,
            image,
            sound,
            track,
            starting_value,
        })
    }

    fn parse_type(&mut self) -> Result<ParameterType, ParsingError> {
        ParameterType::try_from(PrimitiveParser::parse_byte(self.cursor)?)
            .map_err(|_| ParsingError::InvalidParameter(ParameterError::InvalidType))
    }

    fn parse_critical_value(&mut self) -> Result<CriticalValue, ParsingError> {
        match PrimitiveParser::parse_byte(self.cursor)? {
            0x00 => Ok(CriticalValue::Max),
            0x01 => Ok(CriticalValue::Min),
            _ => Err(ParsingError::InvalidParameter(
                ParameterError::InvalidCriticalValue,
            )),
        }
    }

    fn parse_formatted_range_lines(
        &mut self,
        count: usize,
    ) -> Result<Vec<FormattedRangeLine>, ParsingError> {
        let mut parsed = Vec::with_capacity(count);
        let mut iters = 0;

        while iters < count {
            parsed.push(FormattedRangeLine {
                from: PrimitiveParser::parse_i32(self.cursor)?,
                to: PrimitiveParser::parse_i32(self.cursor)?,
                value: StringParser::parse(self.cursor)?,
            });

            iters += 1;
        }

        Ok(parsed)
    }
}
