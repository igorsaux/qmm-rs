#[macro_export]
macro_rules! matches_digit {
    ($ch:expr) => {
        matches!(ch, digit_match!())
    };
}

#[macro_export]
macro_rules! digit_match {
    () => {
        b'0'..=b'9'
    };
}

#[macro_export]
macro_rules! matches_ascii_letter {
    ($ch:expr) => {
        matches!(ch, ascii_letter_match!())
    };
}

#[macro_export]
macro_rules! ascii_letter_match {
    () => {
        b'A'..=b'Z' | b'a'..=b'z'
    };
}
