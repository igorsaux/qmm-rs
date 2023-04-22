use std::io::{Cursor, Read};

use super::{
    CompletionCondition, Header, HeaderError, JumpsLimit, ParsingError, PlanetType, PlayerStatus,
    PrimitiveParser, Race, Version,
};

pub struct HeaderParser;

impl HeaderParser {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Header, ParsingError> {
        let version = Self::parse_version(cursor)?;
        let giver_race = Self::parse_quest_giver_race(cursor)?;
        let completion_condition = Self::parse_completion_condition(cursor)?;
        let quest_planet_type = Self::parse_quest_planet_type(cursor)?;
        let player_status = Self::parse_player_status(cursor)?;
        let player_race = Self::parse_player_race(cursor)?;
        let relation_change = Self::parse_relation_change(cursor)?;

        // Skip screen and grid sizes...
        PrimitiveParser::parse_i32(cursor)?;
        PrimitiveParser::parse_i32(cursor)?;
        PrimitiveParser::parse_i32(cursor)?;
        PrimitiveParser::parse_i32(cursor)?;

        let default_jumps_limit = Self::parse_jumps_limit(cursor)?;
        let difficult = PrimitiveParser::parse_i32(cursor)? as u32;
        let parameters_count = PrimitiveParser::parse_i32(cursor)? as usize;

        Ok(Header {
            version,
            giver_race,
            completion_condition,
            quest_planet_type,
            player_status,
            player_race,
            relation_change,
            default_jumps_limit,
            difficult,
            parameters_count,
        })
    }

    fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<Version, ParsingError> {
        let mut version_bytes = [0; 4];

        cursor
            .read_exact(&mut version_bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        let version = Version::try_from(&[
            version_bytes[0],
            version_bytes[1],
            version_bytes[2],
            version_bytes[3],
        ])
        .map_err(|_| ParsingError::InvalidHeader(HeaderError::InvalidMagic))?;

        let old_pos = cursor.position();
        let mut empty_bytes = [0; 12];

        cursor
            .read_exact(&mut empty_bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        // QMM7 moment
        if empty_bytes
            != [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        {
            cursor.set_position(old_pos);
        }

        Ok(version)
    }

    fn parse_quest_giver_race(cursor: &mut Cursor<&[u8]>) -> Result<Race, ParsingError> {
        Race::try_from(PrimitiveParser::parse_byte(cursor)?)
            .map_err(|_| ParsingError::InvalidHeader(HeaderError::InvalidQuestGiverRace))
    }

    fn parse_completion_condition(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<CompletionCondition, ParsingError> {
        CompletionCondition::try_from(PrimitiveParser::parse_byte(cursor)?)
            .map_err(|_| ParsingError::InvalidHeader(HeaderError::InvalidCompletionCondition))
    }

    fn parse_quest_planet_type(cursor: &mut Cursor<&[u8]>) -> Result<PlanetType, ParsingError> {
        let value = PrimitiveParser::parse_byte(cursor)?;

        match value {
            0x40 => Ok(PlanetType::Uninhabited),
            _ => Race::try_from(value)
                .map(PlanetType::Populated)
                .map_err(|_| ParsingError::InvalidHeader(HeaderError::InvalidQuestPlanetType)),
        }
    }

    fn parse_player_status(cursor: &mut Cursor<&[u8]>) -> Result<PlayerStatus, ParsingError> {
        PlayerStatus::try_from(PrimitiveParser::parse_byte(cursor)?)
            .map_err(|_| ParsingError::InvalidHeader(HeaderError::InvalidPlayerStatus))
    }

    fn parse_player_race(cursor: &mut Cursor<&[u8]>) -> Result<Race, ParsingError> {
        Race::try_from(PrimitiveParser::parse_byte(cursor)?)
            .map_err(|_| ParsingError::InvalidHeader(HeaderError::InvalidPlayerRace))
    }

    fn parse_relation_change(cursor: &mut Cursor<&[u8]>) -> Result<i8, ParsingError> {
        let mut relation_change_bytes = [0; 4];

        cursor
            .read_exact(&mut relation_change_bytes)
            .map_err(|_| ParsingError::Incomplete)?;

        let value = relation_change_bytes[0] as i8;

        match relation_change_bytes {
            [_, 0xFF, 0xFF, 0xFF] => Ok(-value),
            [_, 0x00, 0x00, 0x00] => Ok(value),
            _ => Err(ParsingError::InvalidHeader(
                HeaderError::InvalidRelationChange,
            )),
        }
    }

    fn parse_jumps_limit(cursor: &mut Cursor<&[u8]>) -> Result<JumpsLimit, ParsingError> {
        let value = PrimitiveParser::parse_i32(cursor)? as u32;

        if value == 0x00 {
            Ok(JumpsLimit::Infinite)
        } else {
            Ok(JumpsLimit::Limit(value))
        }
    }
}
