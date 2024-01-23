use std::num;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The message is missing the `$` prefix")]
    MissingPrefix,
    #[error("The length of the input was not long enough")]
    IncorrectLength,
    #[error("The message's checksum did not match")]
    InvalidChecksum,
    #[error("An integer was invalid")]
    InvalidNumber(#[from] num::ParseIntError),
    #[error("A float was invalid")]
    InvalidFloat(#[from] num::ParseFloatError),
    #[error("Unknown message type")]
    UnknownType([u8; 3]),
    #[error("Non UTF-8 character")]
    NonUtf8(#[from] std::str::Utf8Error),
    #[error("Unexpected character")]
    UnexpectedChar(char),
    #[error("Parser has remaining data")]
    RemainingData,
}
