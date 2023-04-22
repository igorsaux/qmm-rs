use std::{fmt::Display, ops::RangeInclusive};

use crate::digit_match;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToRangeValue {
    Parameter { index: usize },
    Integer { value: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormulaTokenKind {
    OpenParenthesis,
    CloseParenthesis,
    Substract,
    Add,
    Multiply,
    Divide,
    DivideWithRemain,
    Modulo,
    In,
    And,
    Or,
    Greater,
    GreaterOrEqual,
    Lesser,
    LesserOrEqual,
    Equal,
    NotEqual,
    Assignment,
    Integer {
        value: i32,
    },
    Double {
        value: f64,
    },
    Parameter {
        value: usize,
    },
    Range {
        value: Vec<RangeInclusive<i32>>,
    },
    ToRange {
        start: ToRangeValue,
        end: ToRangeValue,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormulaToken {
    pub kind: FormulaTokenKind,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Formula {
    pub tokens: Vec<FormulaToken>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormulaErrorKind {
    UnexpectedToken {
        found: char,
        expected: Option<String>,
    },
    ExpectedInteger,
    ExpectedDouble,
    UnexpectedEOF,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormulaError {
    pub position: usize,
    pub kind: FormulaErrorKind,
}

impl Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.position;

        match &self.kind {
            FormulaErrorKind::UnexpectedToken {
                found,
                expected: Some(expected),
            } => f.write_fmt(format_args!(
                "Got unexpected token `{found}` at position {pos}, but expected `{expected}`"
            )),
            FormulaErrorKind::UnexpectedToken {
                found,
                expected: None,
            } => f.write_fmt(format_args!(
                "Got unexpected token `{found}` at position {pos}"
            )),
            FormulaErrorKind::ExpectedInteger => {
                f.write_fmt(format_args!("Expected integer at position {pos}"))
            }
            FormulaErrorKind::ExpectedDouble => {
                f.write_fmt(format_args!("Expected double at position {pos}"))
            }
            FormulaErrorKind::UnexpectedEOF => {
                f.write_fmt(format_args!("Unexpected end of formula at position {pos}"))
            }
        }
    }
}

impl Formula {
    pub fn parse(text: &str) -> Result<Formula, FormulaError> {
        let mut formula = Formula { tokens: Vec::new() };
        let buffer = text.as_bytes();
        let mut pos = 0;

        macro_rules! unexpected {
            ($found:expr, $expected:expr) => {
                return Err(FormulaError {
                    position: pos,
                    kind: FormulaErrorKind::UnexpectedToken {
                        found: $found,
                        expected: Some($expected),
                    },
                })
            };
            ($found:expr) => {
                return Err(FormulaError {
                    position: pos,
                    kind: FormulaErrorKind::UnexpectedToken {
                        found: $found,
                        expected: None,
                    },
                })
            };
        }

        while pos < buffer.len() {
            let ch = buffer[pos];

            match ch {
                b'(' => formula.tokens.push(FormulaToken {
                    kind: FormulaTokenKind::OpenParenthesis,
                    value: "(".to_string(),
                }),
                b')' => formula.tokens.push(FormulaToken {
                    kind: FormulaTokenKind::CloseParenthesis,
                    value: ")".to_string(),
                }),
                b'-' => match Self::try_parse_number(buffer, pos) {
                    Some(Err(err)) => return Err(err),
                    Some(Ok(token)) => {
                        pos += token.value.len() - 1;
                        formula.tokens.push(token);
                    }
                    None => formula.tokens.push(FormulaToken {
                        kind: FormulaTokenKind::Substract,
                        value: "-".to_string(),
                    }),
                },
                b'+' => formula.tokens.push(FormulaToken {
                    kind: FormulaTokenKind::Add,
                    value: "+".to_string(),
                }),
                b'*' => formula.tokens.push(FormulaToken {
                    kind: FormulaTokenKind::Multiply,
                    value: "*".to_string(),
                }),
                b'/' => formula.tokens.push(FormulaToken {
                    kind: FormulaTokenKind::Divide,
                    value: "/".to_string(),
                }),
                b'd' => {
                    if Self::try_parse_word("div", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::DivideWithRemain,
                            value: "div".to_string(),
                        });

                        pos += 2;
                    } else {
                        unexpected!('d', "div".to_string());
                    }
                }
                b'm' => {
                    if Self::try_parse_word("mod", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::Modulo,
                            value: "mod".to_string(),
                        });

                        pos += 2;
                    } else {
                        unexpected!('m', "mod".to_string());
                    }
                }
                b'a' => {
                    if Self::try_parse_word("and", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::And,
                            value: "and".to_string(),
                        });

                        pos += 2;
                    } else {
                        unexpected!('a', "and".to_string());
                    }
                }
                b'o' => {
                    if Self::try_parse_word("or", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::Or,
                            value: "or".to_string(),
                        });

                        pos += 1;
                    } else {
                        unexpected!('o', "or".to_string());
                    }
                }
                b'i' => {
                    if Self::try_parse_word("in", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::In,
                            value: "in".to_string(),
                        });

                        pos += 1;
                    } else {
                        unexpected!('i', "in".to_string());
                    }
                }
                b'>' => {
                    if Self::try_parse_word(">=", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::GreaterOrEqual,
                            value: ">=".to_string(),
                        });

                        pos += 1;
                    } else {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::Greater,
                            value: ">".to_string(),
                        })
                    }
                }
                b'<' => {
                    if Self::try_parse_word("<=", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::LesserOrEqual,
                            value: "<=".to_string(),
                        });

                        pos += 1;
                    } else if Self::try_parse_word("<>", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::NotEqual,
                            value: "<>".to_string(),
                        });

                        pos += 1;
                    } else {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::Lesser,
                            value: "<".to_string(),
                        })
                    }
                }
                b'=' => {
                    if Self::try_parse_word("==", buffer, pos) {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::Equal,
                            value: "==".to_string(),
                        });

                        pos += 1;
                    } else {
                        formula.tokens.push(FormulaToken {
                            kind: FormulaTokenKind::Assignment,
                            value: "=".to_string(),
                        })
                    }
                }
                digit_match!() => {
                    let Some(token) = Self::try_parse_to_range(buffer, pos).or_else(|| Self::try_parse_number(buffer, pos)) else {
                        return Err(FormulaError { position: pos, kind: FormulaErrorKind::ExpectedInteger });
                    };

                    let token = token?;

                    pos += token.value.len() - 1;
                    formula.tokens.push(token);
                }
                b'[' => {
                    let Some(token) = Self::try_parse_range(buffer, pos).or_else(|| Self::try_parse_to_range(buffer, pos).or_else(|| Self::try_parse_parameter(buffer, pos))) else {
                        unexpected!('[')
                    };

                    let token = token?;

                    pos += token.value.len() - 1;
                    formula.tokens.push(token);
                }
                b' ' => (),
                _ => {
                    unexpected!(ch as char);
                }
            }

            pos += 1;
        }

        Ok(formula)
    }

    pub fn try_parse_to_range(
        buffer: &[u8],
        start: usize,
    ) -> Option<Result<FormulaToken, FormulaError>> {
        let mut pos = start;
        let start_range;
        let end_range;

        match Self::try_parse_parameter(buffer, pos).or_else(|| Self::try_parse_number(buffer, pos))
        {
            Some(Ok(FormulaToken {
                kind: FormulaTokenKind::Parameter { value: index },
                value,
            })) => {
                pos += value.len();
                start_range = ToRangeValue::Parameter { index }
            }
            Some(Ok(FormulaToken {
                kind: FormulaTokenKind::Integer { value: int },
                value,
            })) => {
                pos += value.len();
                start_range = ToRangeValue::Integer { value: int }
            }
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(FormulaError {
                    position: pos,
                    kind: FormulaErrorKind::UnexpectedToken {
                        found: buffer[pos] as char,
                        expected: None,
                    },
                }))
            }
            _ => return None,
        };

        while pos < buffer.len() {
            if buffer[pos] == b' ' {
                pos += 1;

                continue;
            } else {
                break;
            }
        }

        if pos == buffer.len() {
            return None;
        }

        if !matches!(buffer.get(pos..=pos + 1), Some([b't', b'o'])) {
            return None;
        }

        pos += 2;

        while pos < buffer.len() {
            if buffer[pos] == b' ' {
                pos += 1;

                continue;
            } else {
                break;
            }
        }

        if pos == buffer.len() {
            return Some(Err(FormulaError {
                position: pos,
                kind: FormulaErrorKind::UnexpectedEOF,
            }));
        }

        match Self::try_parse_parameter(buffer, pos).or_else(|| Self::try_parse_number(buffer, pos))
        {
            Some(Ok(FormulaToken {
                kind: FormulaTokenKind::Parameter { value: index },
                value,
            })) => {
                pos += value.len();
                end_range = ToRangeValue::Parameter { index }
            }
            Some(Ok(FormulaToken {
                kind: FormulaTokenKind::Integer { value: int },
                value,
            })) => {
                pos += value.len();
                end_range = ToRangeValue::Integer { value: int }
            }
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(FormulaError {
                    position: pos,
                    kind: FormulaErrorKind::UnexpectedToken {
                        found: buffer[pos] as char,
                        expected: None,
                    },
                }))
            }
            _ => {
                return Some(Err(FormulaError {
                    position: pos,
                    kind: FormulaErrorKind::UnexpectedToken {
                        found: buffer[pos] as char,
                        expected: None,
                    },
                }))
            }
        };

        let value_bytes = buffer[start..=pos - 1].to_vec();

        Some(Ok(FormulaToken {
            kind: FormulaTokenKind::ToRange {
                start: start_range,
                end: end_range,
            },
            value: String::from_utf8(value_bytes).unwrap(),
        }))
    }

    pub fn try_parse_range(
        buffer: &[u8],
        start: usize,
    ) -> Option<Result<FormulaToken, FormulaError>> {
        let mut pos = start;

        if !matches!(buffer[pos], b'[') {
            return None;
        }

        pos += 1;

        let mut ranges = Vec::new();

        if matches!(buffer.get(pos), Some(b']')) {
            return Some(Err(FormulaError {
                position: pos,
                kind: FormulaErrorKind::ExpectedInteger,
            }));
        }

        while pos < buffer.len() {
            let start_range = match Self::try_parse_number(buffer, pos) {
                Some(Err(err)) => return Some(Err(err)),
                Some(Ok(token)) => {
                    pos += token.value.len();

                    match token.kind {
                        FormulaTokenKind::Integer { value } => value,
                        _ => {
                            return Some(Err(FormulaError {
                                position: pos - token.value.len(),
                                kind: FormulaErrorKind::ExpectedInteger,
                            }))
                        }
                    }
                }
                None => {
                    return None;
                }
            };

            if matches!(buffer.get(pos..=pos + 1), Some([b'.', b'.'])) {
                pos += 2;

                let end_range = match Self::try_parse_number(buffer, pos) {
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(token)) => {
                        pos += token.value.len();

                        match token.kind {
                            FormulaTokenKind::Integer { value } => value,
                            _ => {
                                return Some(Err(FormulaError {
                                    position: pos - token.value.len(),
                                    kind: FormulaErrorKind::ExpectedInteger,
                                }));
                            }
                        }
                    }
                    None => {
                        return Some(Err(FormulaError {
                            position: pos,
                            kind: FormulaErrorKind::ExpectedInteger,
                        }))
                    }
                };

                ranges.push(start_range..=end_range);

                match buffer.get(pos) {
                    Some(b']') => break,
                    Some(b';') => {
                        pos += 1;
                        continue;
                    }
                    _ => {
                        return Some(Err(FormulaError {
                            position: pos,
                            kind: FormulaErrorKind::UnexpectedToken {
                                found: buffer[pos] as char,
                                expected: Some(";".to_string()),
                            },
                        }));
                    }
                }
            }

            let Some(ch) = buffer.get(pos) else {
                return Some(Err(FormulaError { position: pos, kind: FormulaErrorKind::UnexpectedEOF }));
            };

            match ch {
                b';' => {
                    ranges.push(start_range..=start_range);
                }
                b']' => break,
                _ => {
                    return Some(Err(FormulaError {
                        position: pos,
                        kind: FormulaErrorKind::UnexpectedToken {
                            found: *ch as char,
                            expected: Some("; or ]".to_string()),
                        },
                    }))
                }
            }

            pos += 1;
        }

        if !matches!(buffer.get(pos), Some(b']')) {
            return Some(Err(FormulaError {
                position: pos,
                kind: FormulaErrorKind::UnexpectedToken {
                    found: buffer[pos] as char,
                    expected: Some("]".to_string()),
                },
            }));
        }

        let string_bytes = buffer[start..=pos].to_vec();
        let string = String::from_utf8(string_bytes).unwrap();

        Some(Ok(FormulaToken {
            kind: FormulaTokenKind::Range { value: ranges },
            value: string,
        }))
    }

    pub fn try_parse_parameter(
        buffer: &[u8],
        start: usize,
    ) -> Option<Result<FormulaToken, FormulaError>> {
        let mut pos = start;

        if !matches!(buffer.get(pos..=pos + 1), Some([b'[', b'p'])) {
            return None;
        }

        pos += 2;

        let number = match Self::try_parse_number(buffer, pos) {
            Some(Err(err)) => return Some(Err(err)),
            Some(Ok(token)) => {
                pos += token.value.len();

                match token.kind {
                    FormulaTokenKind::Integer { value } => value,
                    _ => {
                        return Some(Err(FormulaError {
                            position: pos - token.value.len(),
                            kind: FormulaErrorKind::ExpectedInteger,
                        }))
                    }
                }
            }
            None => {
                return Some(Err(FormulaError {
                    position: pos,
                    kind: FormulaErrorKind::ExpectedInteger,
                }))
            }
        };

        if !matches!(buffer.get(pos), Some(b']')) {
            return Some(Err(FormulaError {
                position: pos,
                kind: FormulaErrorKind::UnexpectedToken {
                    found: buffer[pos] as char,
                    expected: Some("]".to_string()),
                },
            }));
        }

        let string_bytes = buffer[start..=pos].to_vec();
        let string = String::from_utf8(string_bytes).unwrap();

        Some(Ok(FormulaToken {
            kind: FormulaTokenKind::Parameter {
                value: number as usize,
            },
            value: string,
        }))
    }

    pub fn try_parse_number(
        buffer: &[u8],
        start: usize,
    ) -> Option<Result<FormulaToken, FormulaError>> {
        let mut pos = start;
        let mut is_double = false;

        while pos < buffer.len() {
            let ch = buffer[pos];

            match ch {
                digit_match!() => {}
                b'.' => {
                    if matches!(buffer.get(pos + 1), Some(b'.')) {
                        break;
                    }

                    if is_double {
                        return Some(Err(FormulaError {
                            position: pos,
                            kind: FormulaErrorKind::UnexpectedToken {
                                found: '.',
                                expected: None,
                            },
                        }));
                    }

                    is_double = true;
                }
                b'-' => {
                    if pos != start {
                        break;
                    }
                }
                _ => break,
            }

            pos += 1;
        }

        let number_bytes = buffer[start..pos].to_vec();
        let number_string = String::from_utf8(number_bytes).unwrap();

        if is_double {
            let Ok(number) = number_string.parse::<f64>() else {
                return None;
            };

            Some(Ok(FormulaToken {
                kind: FormulaTokenKind::Double { value: number },
                value: number_string,
            }))
        } else {
            let Ok(number) = number_string.parse::<i32>() else {
                return None;
            };

            Some(Ok(FormulaToken {
                kind: FormulaTokenKind::Integer { value: number },
                value: number_string,
            }))
        }
    }

    pub fn try_parse_word(word: &str, buffer: &[u8], start: usize) -> bool {
        let mut pos = start;
        let mut relative = 0;

        while pos < buffer.len() {
            let ch = buffer[pos];

            if relative == word.len() {
                return true;
            }

            let Some(target_ch) = word.as_bytes().get(relative).copied() else {
                return false;
            };

            if ch != target_ch {
                return false;
            }

            relative += 1;
            pos += 1;
        }

        relative == word.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::text::formula::{Formula, FormulaToken, FormulaTokenKind, ToRangeValue};

    #[test]
    pub fn parse_open_parenthesis() {
        assert_eq!(
            Formula::parse("(").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::OpenParenthesis,
                    value: "(".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_close_parenthesis() {
        assert_eq!(
            Formula::parse(")").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::CloseParenthesis,
                    value: ")".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_substract() {
        assert_eq!(
            Formula::parse("-").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Substract,
                    value: "-".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_add() {
        assert_eq!(
            Formula::parse("+").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Add,
                    value: "+".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_multiply() {
        assert_eq!(
            Formula::parse("*").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Multiply,
                    value: "*".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_divide() {
        assert_eq!(
            Formula::parse("/").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Divide,
                    value: "/".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_divide_with_remain() {
        assert_eq!(
            Formula::parse("div").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::DivideWithRemain,
                    value: "div".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_modulo() {
        assert_eq!(
            Formula::parse("mod").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Modulo,
                    value: "mod".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_in() {
        assert_eq!(
            Formula::parse("in").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::In,
                    value: "in".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_and() {
        assert_eq!(
            Formula::parse("and").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::And,
                    value: "and".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_or() {
        assert_eq!(
            Formula::parse("or").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Or,
                    value: "or".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_greater() {
        assert_eq!(
            Formula::parse(">").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Greater,
                    value: ">".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_greater_or_equal() {
        assert_eq!(
            Formula::parse(">=").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::GreaterOrEqual,
                    value: ">=".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_lesser() {
        assert_eq!(
            Formula::parse("<").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Lesser,
                    value: "<".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_lesser_or_equal() {
        assert_eq!(
            Formula::parse("<=").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::LesserOrEqual,
                    value: "<=".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_equal() {
        assert_eq!(
            Formula::parse("==").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Equal,
                    value: "==".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_not_equal() {
        assert_eq!(
            Formula::parse("<>").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::NotEqual,
                    value: "<>".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_assignment() {
        assert_eq!(
            Formula::parse("=").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Assignment,
                    value: "=".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_int() {
        assert_eq!(
            Formula::parse("12345").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Integer { value: 12345 },
                    value: "12345".to_string()
                }]
            }
        );

        assert_eq!(
            Formula::parse("-12345").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Integer { value: -12345 },
                    value: "-12345".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_double() {
        assert_eq!(
            Formula::parse("1.23456").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Double { value: 1.23456f64 },
                    value: "1.23456".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_parameter() {
        assert_eq!(
            Formula::parse("[p123]").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Parameter { value: 123 },
                    value: "[p123]".to_string()
                }]
            }
        );

        assert_eq!(
            Formula::parse("[p123] [p321]").unwrap(),
            Formula {
                tokens: vec![
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 123 },
                        value: "[p123]".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 321 },
                        value: "[p321]".to_string()
                    }
                ]
            }
        )
    }

    #[test]
    pub fn parse_range() {
        assert_eq!(
            Formula::parse("[0..1]").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Range { value: vec![0..=1] },
                    value: "[0..1]".to_string()
                }]
            }
        );

        assert_eq!(
            Formula::parse("[0..1;2;3..4]").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::Range {
                        value: vec![0..=1, 2..=2, 3..=4]
                    },
                    value: "[0..1;2;3..4]".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_to_range() {
        assert_eq!(
            Formula::parse("0 to 1").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::ToRange {
                        start: ToRangeValue::Integer { value: 0 },
                        end: ToRangeValue::Integer { value: 1 }
                    },
                    value: "0 to 1".to_string()
                }]
            }
        );

        assert_eq!(
            Formula::parse("[p0] to [p1]").unwrap(),
            Formula {
                tokens: vec![FormulaToken {
                    kind: FormulaTokenKind::ToRange {
                        start: ToRangeValue::Parameter { index: 0 },
                        end: ToRangeValue::Parameter { index: 1 }
                    },
                    value: "[p0] to [p1]".to_string()
                }]
            }
        );

        assert_eq!(
            Formula::parse("[p0] to 1 * 2 to [p1]").unwrap(),
            Formula {
                tokens: vec![
                    FormulaToken {
                        kind: FormulaTokenKind::ToRange {
                            start: ToRangeValue::Parameter { index: 0 },
                            end: ToRangeValue::Integer { value: 1 }
                        },
                        value: "[p0] to 1".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Multiply,
                        value: "*".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::ToRange {
                            start: ToRangeValue::Integer { value: 2 },
                            end: ToRangeValue::Parameter { index: 1 }
                        },
                        value: "2 to [p1]".to_string()
                    }
                ]
            }
        )
    }

    #[test]
    pub fn parse_expressions() {
        assert_eq!(
            Formula::parse("(([p8] div 2) mod 2)=0").unwrap(),
            Formula {
                tokens: vec![
                    FormulaToken {
                        kind: FormulaTokenKind::OpenParenthesis,
                        value: "(".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::OpenParenthesis,
                        value: "(".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 8 },
                        value: "[p8]".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::DivideWithRemain,
                        value: "div".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Integer { value: 2 },
                        value: "2".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::CloseParenthesis,
                        value: ")".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Modulo,
                        value: "mod".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Integer { value: 2 },
                        value: "2".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::CloseParenthesis,
                        value: ")".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Assignment,
                        value: "=".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Integer { value: 0 },
                        value: "0".to_string()
                    }
                ]
            }
        );

        assert_eq!(
            Formula::parse("[p1] >= ([p2]+1) * [p15]/[p7]").unwrap(),
            Formula {
                tokens: vec![
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 1 },
                        value: "[p1]".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::GreaterOrEqual,
                        value: ">=".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::OpenParenthesis,
                        value: "(".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 2 },
                        value: "[p2]".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Add,
                        value: "+".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Integer { value: 1 },
                        value: "1".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::CloseParenthesis,
                        value: ")".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Multiply,
                        value: "*".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 15 },
                        value: "[p15]".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Divide,
                        value: "/".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 7 },
                        value: "[p7]".to_string()
                    }
                ]
            }
        );

        assert_eq!(
            Formula::parse("2-([p8] mod 2)").unwrap(),
            Formula {
                tokens: vec![
                    FormulaToken {
                        kind: FormulaTokenKind::Integer { value: 2 },
                        value: "2".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Substract,
                        value: "-".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::OpenParenthesis,
                        value: "(".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Parameter { value: 8 },
                        value: "[p8]".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Modulo,
                        value: "mod".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::Integer { value: 2 },
                        value: "2".to_string()
                    },
                    FormulaToken {
                        kind: FormulaTokenKind::CloseParenthesis,
                        value: ")".to_string()
                    }
                ]
            }
        )
    }
}
