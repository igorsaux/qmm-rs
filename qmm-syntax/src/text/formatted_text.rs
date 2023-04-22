use std::fmt::Display;

use crate::digit_match;

static VARIABLES: [&str; 8] = [
    "<ToStar>",
    "<ToPlanet>",
    "<FromStar>",
    "<FromPlanet>",
    "<Ranger>",
    "<Date>",
    "<Day>",
    "<Money>",
];

static CLR_BEGIN_TAG: &str = "<clr>";
static CLR_END_TAG: &str = "<clrEnd>";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextElementKind {
    Text,
    /// `<ToStar>`, `<ToPlanet>`
    Variable {
        name: String,
    },
    /// `{[p1] mod 1}`
    Formula {
        text: String,
    },
    /// `<>`
    CurrentParameter,
    /// `\n`, `\r\n`
    NewLine,
    /// `<clr>Foo<clrEnd>`,
    Selection {
        text: String,
    },
    /// `[p1]`
    Parameter {
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextElement {
    pub kind: TextElementKind,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormattedText {
    pub elements: Vec<TextElement>,
}

impl Display for FormattedText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for el in self.elements.iter() {
            f.write_str(&el.value)?;
        }

        Ok(())
    }
}

impl FormattedText {
    pub fn parse(text: &str) -> FormattedText {
        let mut elements = Vec::new();

        if text.is_empty() {
            return FormattedText { elements };
        }

        let buffer = text.as_bytes();
        let mut pos = 0;
        let mut last_el_pos = 0;

        fn push_text_from_prev_el(
            last_el_pos: usize,
            pos: usize,
            buffer: &[u8],
            elements: &mut Vec<TextElement>,
        ) {
            if last_el_pos != buffer.len() && last_el_pos != pos {
                let text_bytes = buffer[last_el_pos..pos].to_vec();
                elements.push(TextElement {
                    kind: TextElementKind::Text,
                    value: String::from_utf8(text_bytes).unwrap(),
                });
            }
        }

        while pos < buffer.len() {
            let ch = buffer[pos];

            match ch {
                b'<' => {
                    if let Some(el) = Self::try_parse_variable(buffer, pos)
                        .or_else(|| Self::try_parse_current_parameter(buffer, pos))
                        .or_else(|| Self::try_parse_text_selection(buffer, pos))
                    {
                        push_text_from_prev_el(last_el_pos, pos, buffer, &mut elements);

                        pos += el.value.len();
                        last_el_pos = pos;
                        elements.push(el);
                    }
                }
                b'{' => {
                    if let Some(el) = Self::try_parse_formula(buffer, pos) {
                        push_text_from_prev_el(last_el_pos, pos, buffer, &mut elements);

                        pos += el.value.len();
                        last_el_pos = pos;
                        elements.push(el);
                    }
                }
                b'\n' => {
                    let el = TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\n".to_string(),
                    };

                    push_text_from_prev_el(last_el_pos, pos, buffer, &mut elements);

                    pos += el.value.len();
                    last_el_pos = pos;
                    elements.push(el);
                }
                b'\r' => {
                    if !matches!(buffer.get(pos + 1), Some(b'\n')) {
                        continue;
                    }

                    let el = TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string(),
                    };

                    push_text_from_prev_el(last_el_pos, pos, buffer, &mut elements);

                    pos += el.value.len();
                    last_el_pos = pos;
                    elements.push(el);

                    continue;
                }
                b'[' => {
                    if let Some(el) = Self::try_parse_parameter(buffer, pos) {
                        push_text_from_prev_el(last_el_pos, pos, buffer, &mut elements);

                        pos += el.value.len();
                        last_el_pos = pos;
                        elements.push(el);
                    }
                }
                _ => (),
            };

            pos += 1;
        }

        push_text_from_prev_el(last_el_pos, pos, buffer, &mut elements);

        FormattedText { elements }
    }

    pub fn try_parse_parameter(buffer: &[u8], start: usize) -> Option<TextElement> {
        if !matches!(buffer.get(start + 1), Some(b'p')) {
            return None;
        }

        let number_start = start + 2;
        let mut pos = number_start;

        while pos < buffer.len() {
            let Some(ch) = buffer.get(pos) else {
                return None;
            };

            match ch {
                b']' => {
                    if pos == number_start {
                        return None;
                    }

                    let index_bytes = buffer[number_start..pos].to_vec();
                    let index_string = String::from_utf8(index_bytes).unwrap();

                    let Ok(index) = index_string.parse::<usize>() else {
                        return None;
                    };

                    let value_bytes = buffer[start..=pos].to_vec();

                    return Some(TextElement {
                        kind: TextElementKind::Parameter { index },
                        value: String::from_utf8(value_bytes).unwrap(),
                    });
                }
                digit_match!() => (),
                _ => return None,
            }

            pos += 1;
        }

        None
    }

    pub fn try_parse_formula(buffer: &[u8], start: usize) -> Option<TextElement> {
        let mut pos = start;
        let text_start = start + 1;

        while pos < buffer.len() {
            let ch = buffer[pos];

            if ch == b'}' {
                let text_bytes = buffer[text_start..pos].to_vec();
                let text = String::from_utf8(text_bytes).unwrap();

                let var_bytes = buffer[start..=pos].to_vec();

                return Some(TextElement {
                    kind: TextElementKind::Formula { text },
                    value: String::from_utf8(var_bytes).unwrap(),
                });
            }

            pos += 1;
        }

        None
    }

    pub fn try_parse_current_parameter(buffer: &[u8], start: usize) -> Option<TextElement> {
        let ch = buffer[start];
        let Some(next_ch) = buffer.get(start + 1).copied() else {
            return None;
        };

        if matches!([ch, next_ch], [b'<', b'>']) {
            Some(TextElement {
                kind: TextElementKind::CurrentParameter,
                value: "<>".to_string(),
            })
        } else {
            None
        }
    }

    pub fn try_parse_text_selection(buffer: &[u8], start: usize) -> Option<TextElement> {
        let Some(begin_tag_end) = Self::try_parse_text_selection_begin_tag_end(buffer, start) else {
            return None;
        };

        let text_start = begin_tag_end + 1;
        let mut pos = text_start;

        while pos < buffer.len() {
            let ch = buffer[pos];

            if ch == b'<' {
                let Some(end_tag_end) = Self::try_parse_text_selection_end_tag_end(buffer, pos) else {
                    return None;
                };

                let text = if text_start == pos {
                    String::new()
                } else {
                    let text_bytes = buffer[text_start..pos].to_vec();
                    String::from_utf8(text_bytes).unwrap()
                };

                let tag_bytes = buffer[start..=end_tag_end].to_vec();

                return Some(TextElement {
                    kind: TextElementKind::Selection { text },
                    value: String::from_utf8(tag_bytes).unwrap(),
                });
            }

            pos += 1;
        }

        None
    }

    pub fn try_parse_text_selection_begin_tag_end(buffer: &[u8], start: usize) -> Option<usize> {
        let begin_tag_end = start + CLR_BEGIN_TAG.len() - 1;

        let Some(clr_begin) = buffer.get(start..=begin_tag_end) else {
            return None;
        };

        if clr_begin == CLR_BEGIN_TAG.as_bytes() {
            Some(begin_tag_end)
        } else {
            None
        }
    }

    pub fn try_parse_text_selection_end_tag_end(buffer: &[u8], start: usize) -> Option<usize> {
        let end_tag_end = start + CLR_END_TAG.len() - 1;

        let Some(clr_begin) = buffer.get(start..=end_tag_end) else {
            return None;
        };

        if clr_begin == CLR_END_TAG.as_bytes() {
            Some(end_tag_end)
        } else {
            None
        }
    }

    pub fn try_parse_variable(buffer: &[u8], start: usize) -> Option<TextElement> {
        let mut pos = start;
        let mut rel_pos = 0;
        let name_start = start + 1;

        while pos < buffer.len() {
            let ch = buffer[pos];

            let do_matches_any_var = VARIABLES.iter().any(|var| {
                if let Some(var_ch) = var.as_bytes().get(rel_pos) {
                    *var_ch == ch
                } else {
                    false
                }
            });

            if !do_matches_any_var {
                return None;
            }

            if ch == b'>' {
                let name_bytes = buffer[name_start..pos].to_vec();
                let name = String::from_utf8(name_bytes).unwrap();

                let var_bytes = buffer[start..=pos].to_vec();

                return Some(TextElement {
                    kind: TextElementKind::Variable { name },
                    value: String::from_utf8(var_bytes).unwrap(),
                });
            }

            pos += 1;
            rel_pos += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::text::formatted_text::{TextElement, TextElementKind};

    use super::FormattedText;

    #[test]
    pub fn parse_default_text() {
        let text = "lorem ipsum";
        let parsed = FormattedText::parse(text);

        assert_eq!(
            parsed,
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Text,
                    value: text.to_string(),
                }]
            }
        )
    }

    #[test]
    pub fn parse_empty_text() {
        let parsed = FormattedText::parse("");

        assert_eq!(parsed, FormattedText { elements: vec![] })
    }

    #[test]
    pub fn parse_not_enclosed_var() {
        let text = "lorem <ipsum";
        let parsed = FormattedText::parse(text);

        assert_eq!(
            parsed,
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Text,
                    value: text.to_string(),
                }]
            }
        )
    }

    #[test]
    pub fn parse_invalid_var() {
        let text = "lorem <ipsum>";
        let parsed = FormattedText::parse(text);

        assert_eq!(
            parsed,
            FormattedText {
                elements: vec![TextElement {
                    value: text.to_string(),
                    kind: TextElementKind::Text,
                }]
            }
        )
    }

    #[test]
    pub fn parse_parameter_placeholder() {
        let text = "lorem <>";
        let parsed = FormattedText::parse(text);

        assert_eq!(
            parsed,
            FormattedText {
                elements: vec![
                    TextElement {
                        value: "lorem ".to_string(),
                        kind: TextElementKind::Text,
                    },
                    TextElement {
                        value: "<>".to_string(),
                        kind: TextElementKind::CurrentParameter
                    }
                ]
            }
        )
    }

    #[test]
    pub fn parse_var() {
        assert_eq!(
            FormattedText::parse("<ToStar> <Ranger>"),
            FormattedText {
                elements: vec![
                    TextElement {
                        value: "<ToStar>".to_string(),
                        kind: TextElementKind::Variable {
                            name: "ToStar".to_string()
                        }
                    },
                    TextElement {
                        value: " ".to_string(),
                        kind: TextElementKind::Text
                    },
                    TextElement {
                        value: "<Ranger>".to_string(),
                        kind: TextElementKind::Variable {
                            name: "Ranger".to_string()
                        }
                    }
                ]
            }
        );
    }

    #[test]
    pub fn parse_var2() {
        assert_eq!(
            FormattedText::parse("lorem <ToStar> ipsum"),
            FormattedText {
                elements: vec![
                    TextElement {
                        value: "lorem ".to_string(),
                        kind: TextElementKind::Text,
                    },
                    TextElement {
                        value: "<ToStar>".to_string(),
                        kind: TextElementKind::Variable {
                            name: "ToStar".to_string()
                        },
                    },
                    TextElement {
                        value: " ipsum".to_string(),
                        kind: TextElementKind::Text,
                    }
                ]
            }
        );
    }

    #[test]
    pub fn parse_empty_formula() {
        assert_eq!(
            FormattedText::parse("{} lorem ipsum"),
            FormattedText {
                elements: vec![
                    TextElement {
                        value: "{}".to_string(),
                        kind: TextElementKind::Formula {
                            text: "".to_string()
                        }
                    },
                    TextElement {
                        value: " lorem ipsum".to_string(),
                        kind: TextElementKind::Text
                    }
                ]
            }
        );

        assert_eq!(
            FormattedText::parse("lorem {} ipsum"),
            FormattedText {
                elements: vec![
                    TextElement {
                        value: "lorem ".to_string(),
                        kind: TextElementKind::Text,
                    },
                    TextElement {
                        value: "{}".to_string(),
                        kind: TextElementKind::Formula {
                            text: "".to_string()
                        },
                    },
                    TextElement {
                        value: " ipsum".to_string(),
                        kind: TextElementKind::Text,
                    }
                ]
            }
        );
    }

    #[test]
    pub fn parse_formula() {
        assert_eq!(
            FormattedText::parse("lorem {[p1] mod x} ipsum"),
            FormattedText {
                elements: vec![
                    TextElement {
                        value: "lorem ".to_string(),
                        kind: TextElementKind::Text,
                    },
                    TextElement {
                        value: "{[p1] mod x}".to_string(),
                        kind: TextElementKind::Formula {
                            text: "[p1] mod x".to_string()
                        },
                    },
                    TextElement {
                        value: " ipsum".to_string(),
                        kind: TextElementKind::Text,
                    }
                ]
            }
        )
    }

    #[test]
    pub fn parse_new_lines() {
        assert_eq!(
            FormattedText::parse("lorem\nipsum"),
            FormattedText {
                elements: vec![
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "lorem".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "ipsum".to_string()
                    }
                ]
            }
        )
    }

    #[test]
    pub fn parse_text_selection() {
        assert_eq!(
            FormattedText::parse("<clr><clrEnd>"),
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Selection {
                        text: String::new()
                    },
                    value: "<clr><clrEnd>".to_string()
                }]
            }
        );

        assert_eq!(
            FormattedText::parse("<clr>lorem<clrEnd>"),
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Selection {
                        text: "lorem".to_string()
                    },
                    value: "<clr>lorem<clrEnd>".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_invalid_text_selection() {
        assert_eq!(
            FormattedText::parse("<clr>"),
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Text,
                    value: "<clr>".to_string()
                }]
            }
        );

        assert_eq!(
            FormattedText::parse("<clr><clr>"),
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Text,
                    value: "<clr><clr>".to_string()
                }]
            }
        );

        assert_eq!(
            FormattedText::parse("<clr><endClr>"),
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Text,
                    value: "<clr><endClr>".to_string()
                }]
            }
        )
    }

    #[test]
    pub fn parse_parameter() {
        assert_eq!(
            FormattedText::parse("[p0],[p11]"),
            FormattedText {
                elements: vec![
                    TextElement {
                        kind: TextElementKind::Parameter { index: 0 },
                        value: "[p0]".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: ",".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Parameter { index: 11 },
                        value: "[p11]".to_string()
                    }
                ]
            }
        )
    }

    #[test]
    pub fn parse_invalid_parameter() {
        assert_eq!(
            FormattedText::parse("[p] [p ] [] [pp] [p-]",),
            FormattedText {
                elements: vec![TextElement {
                    kind: TextElementKind::Text,
                    value: "[p] [p ] [] [pp] [p-]".to_string()
                }]
            }
        )
    }
}
