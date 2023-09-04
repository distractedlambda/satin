use thiserror::Error;

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum Error {
    #[default]
    #[error("unexpected byte")]
    UnexpectedByte,

    #[error("unclosed string literal")]
    UnclosedStringLiteral,

    #[error("unclosed long comment")]
    UnclosedLongComment,

    #[error("unclosed long literal")]
    UnclosedLongLiteral,

    #[error("invalid decimal escape sequence in string literal")]
    InvalidDecEscape,

    #[error("invalid unicode escape sequence in string literal")]
    InvalidUnicodeEscape,
}
