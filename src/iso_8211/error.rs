use std::num;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Iso8211Error {
    #[error("Incorrect length")]
    IncorrectLength,
    #[error("An integer was invalid")]
    InvalidNumber(#[from] num::ParseIntError),
    #[error("A float was invalid")]
    InvalidFloat(#[from] num::ParseFloatError),
    #[error("Non UTF-8 character")]
    NonUtf8(#[from] std::str::Utf8Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Unexpected byte: {0}")]
    UnexpectedByte(u8),
    #[error("Non UTF-8 encoded character")]
    NonUtf8Char(#[from] std::string::FromUtf8Error),
}
