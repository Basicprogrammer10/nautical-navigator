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
}
