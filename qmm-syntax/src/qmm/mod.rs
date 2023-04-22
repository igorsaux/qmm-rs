mod header_parser;
mod info_parser;
mod jump_parameter_condition_parser;
mod jump_parser;
mod location_parser;
mod media_parser;
mod parameter_change_parser;
mod parameter_parser;
mod parser;
mod primitive_parser;
mod string_parser;
mod string_replacements_parser;
mod types;

use std::io::Cursor;

use header_parser::HeaderParser;
use info_parser::InfoParser;
use jump_parameter_condition_parser::JumpParameterConditionParser;
use jump_parser::JumpParser;
use location_parser::LocationParser;
use media_parser::MediaParser;
use parameter_change_parser::ParameterChangeParser;
use parameter_parser::ParameterParser;
pub use parser::QmmParser;
use primitive_parser::PrimitiveParser;
use string_parser::StringParser;
use string_replacements_parser::StringReplacementsParser;

pub use types::*;

pub fn parse_qmm(data: &[u8]) -> Result<Quest, ParsingError> {
    QmmParser::parse(&mut Cursor::new(data))
}
