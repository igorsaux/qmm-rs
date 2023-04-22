use std::io::Cursor;

use super::{Media, ParsingError, StringParser};

pub struct MediaParser;

impl MediaParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Media, ParsingError> {
        let image = StringParser::parse(cursor)?;
        let sound = StringParser::parse(cursor)?;
        let track = StringParser::parse(cursor)?;

        Ok(Media {
            image,
            sound,
            track,
        })
    }
}
