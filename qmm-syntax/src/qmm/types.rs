use std::fmt::Display;

use bitflags::bitflags;

use crate::text::{
    formatted_text::FormattedText,
    formula::{Formula, FormulaError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    Qmm6,
    Qmm7,
}

impl TryFrom<&[u8; 4]> for Version {
    type Error = ();

    fn try_from(value: &[u8; 4]) -> Result<Self, Self::Error> {
        match value {
            [0xD6, 0x35, 0x3A, 0x42] => Ok(Version::Qmm6),
            [0xD7, 0x35, 0x3A, 0x42] => Ok(Version::Qmm7),
            _ => Err(()),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Race: u8 {
        const Malok = 0x01;
        const Peleng = 0x02;
        const Human = 0x04;
        const Fay = 0x08;
        const Gaal = 0x10;
    }
}

impl TryFrom<u8> for Race {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Race::from_bits(value).ok_or(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompletionCondition {
    Immediately,
    AfterReturning,
}

impl TryFrom<u8> for CompletionCondition {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CompletionCondition::AfterReturning),
            0x01 => Ok(CompletionCondition::Immediately),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlanetType {
    Populated(Race),
    Uninhabited,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PlayerStatus: u8 {
        const Trader = 0x01;
        const Pirate = 0x02;
        const Warrior = 0x04;
    }
}

impl TryFrom<u8> for PlayerStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        PlayerStatus::from_bits(value).ok_or(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JumpsLimit {
    Infinite,
    Limit(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Header {
    pub version: Version,
    pub giver_race: Race,
    pub completion_condition: CompletionCondition,
    pub quest_planet_type: PlanetType,
    pub player_status: PlayerStatus,
    pub player_race: Race,
    pub relation_change: i8,
    pub default_jumps_limit: JumpsLimit,
    pub difficult: u32,
    pub parameters_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParameterType {
    Ordinary,
    Fail,
    Win,
    Death,
}

impl TryFrom<u8> for ParameterType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ParameterType::Ordinary),
            0x01 => Ok(ParameterType::Fail),
            0x02 => Ok(ParameterType::Win),
            0x03 => Ok(ParameterType::Death),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CriticalValue {
    Min,
    Max,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormattedRangeLine {
    pub from: i32,
    pub to: i32,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub min_value: i32,
    pub max_value: i32,
    pub ty: ParameterType,
    pub show_when_zero: bool,
    pub critical_value: CriticalValue,
    pub is_active: bool,
    pub is_money: bool,
    pub name: String,
    pub formatted_range_lines: Vec<FormattedRangeLine>,
    pub critical_text: String,
    pub image: String,
    pub sound: String,
    pub track: String,
    pub starting_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringReplacements {
    pub to_star: String,
    pub to_planet: String,
    pub from_planet: String,
    pub from_star: String,
    pub ranger: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub locations_count: u32,
    pub jumps_count: u32,
    pub success_text: FormattedText,
    pub task_text: FormattedText,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaxVisits {
    Infinite,
    Limit(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationType {
    Ordinary,
    Starting,
    Empty,
    Success,
    Fail,
    Death,
}

impl TryFrom<u8> for LocationType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(LocationType::Ordinary),
            0x01 => Ok(LocationType::Starting),
            0x02 => Ok(LocationType::Empty),
            0x03 => Ok(LocationType::Success),
            0x04 => Ok(LocationType::Fail),
            0x05 => Ok(LocationType::Death),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParameterShowType {
    Nothing,
    Show,
    Hide,
}

impl TryFrom<u8> for ParameterShowType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ParameterShowType::Nothing),
            0x01 => Ok(ParameterShowType::Show),
            0x02 => Ok(ParameterShowType::Hide),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParameterChangeType {
    Value,
    Sum,
    Percentage,
    Formula,
}

impl TryFrom<u8> for ParameterChangeType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ParameterChangeType::Value),
            0x01 => Ok(ParameterChangeType::Sum),
            0x02 => Ok(ParameterChangeType::Percentage),
            0x03 => Ok(ParameterChangeType::Formula),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Media {
    pub image: String,
    pub sound: String,
    pub track: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterChange {
    pub parameter_id: u32,
    pub show_type: ParameterShowType,
    pub change_type: ParameterChangeType,
    pub formula: Formula,
    pub critical_text: String,
    pub media: Media,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocationSelectType {
    ByOrder,
    ByFormula(Formula),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocationId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub do_pass_day: bool,
    pub id: LocationId,
    pub max_visits: MaxVisits,
    pub ty: LocationType,
    pub parameter_changes: Vec<ParameterChange>,
    pub texts: Vec<FormattedText>,
    pub media: Vec<Media>,
    pub select_type: LocationSelectType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JumpId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JumpParameterCondition {
    pub parameter_id: u32,
    pub range_start: i32,
    pub range_end: i32,
    pub must_equal: bool,
    pub must_equal_values: Vec<i32>,
    pub must_mod: bool,
    pub must_mod_values: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Jump {
    pub priority: f64,
    pub do_pass_day: bool,
    pub id: JumpId,
    pub from: LocationId,
    pub to: LocationId,
    pub show_always: bool,
    pub max_visits: MaxVisits,
    pub show_order: u32,
    pub parameters_conditions: Vec<JumpParameterCondition>,
    pub parameter_changes: Vec<ParameterChange>,
    pub formula: Formula,
    pub text: FormattedText,
    pub description: FormattedText,
    pub media: Media,
}

#[derive(Debug, Clone)]
pub struct Quest {
    pub header: Header,
    pub parameters: Vec<Parameter>,
    pub string_replacements: StringReplacements,
    pub info: Info,
    pub locations: Vec<Location>,
    pub jumps: Vec<Jump>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParsingError {
    InvalidHeader(HeaderError),
    InvalidParameter(ParameterError),
    InvalidLocation(LocationError),
    InvalidParameterChange(ParameterChangeError),
    InvalidBool,
    InvalidString,
    Incomplete,
    ExpectedEnd,
    InvalidFormula {
        error: FormulaError,
        formula: String,
    },
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidHeader(err) => err.fmt(f),
            ParsingError::InvalidParameter(err) => err.fmt(f),
            ParsingError::InvalidLocation(err) => err.fmt(f),
            ParsingError::InvalidParameterChange(err) => err.fmt(f),
            ParsingError::InvalidBool => f.write_str("Invalid bool"),
            ParsingError::InvalidString => f.write_str("Invalid string"),
            ParsingError::Incomplete => f.write_str("Incomplete"),
            ParsingError::ExpectedEnd => f.write_str("Expected end"),
            ParsingError::InvalidFormula { error, formula } => {
                f.write_fmt(format_args!("Formula error in `{formula}`: {error}"))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeaderError {
    InvalidMagic,
    InvalidQuestGiverRace,
    InvalidCompletionCondition,
    InvalidQuestPlanetType,
    InvalidPlayerStatus,
    InvalidPlayerRace,
    InvalidRelationChange,
}

impl Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderError::InvalidMagic => f.write_str("Quest error: invalid file magic"),
            HeaderError::InvalidQuestGiverRace => f.write_str("Quest error: invalid giver's race"),
            HeaderError::InvalidCompletionCondition => {
                f.write_str("Quest error: invalid completion condition")
            }
            HeaderError::InvalidQuestPlanetType => f.write_str("Quest error: invalid planet type"),
            HeaderError::InvalidPlayerStatus => f.write_str("Quest error: invalid player status"),
            HeaderError::InvalidPlayerRace => f.write_str("Quest error: invalid player's race"),
            HeaderError::InvalidRelationChange => {
                f.write_str("Quest error: invalid relation change")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParameterError {
    InvalidType,
    InvalidCriticalValue,
}

impl Display for ParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterError::InvalidType => f.write_str("Parameter error: invalid type"),
            ParameterError::InvalidCriticalValue => {
                f.write_str("Parameter error: invalid critical value")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationError {
    InvalidLocationType,
}

impl Display for LocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationError::InvalidLocationType => f.write_str("Location error: Invalid type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParameterChangeError {
    InvalidShowType,
    InvalidChangeType,
}

impl Display for ParameterChangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterChangeError::InvalidShowType => {
                f.write_str("Parameter change error: invalid show type")
            }
            ParameterChangeError::InvalidChangeType => {
                f.write_str("Parameter change error: invalid type")
            }
        }
    }
}
