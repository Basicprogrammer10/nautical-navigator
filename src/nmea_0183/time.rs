use std::{fmt::Debug, str};

use crate::misc::parser::{FromParser, Parser};

use super::ParseError;

/// In UTC.
#[derive(Clone, Copy)]
pub struct Time {
    hour: u8,
    min: u8,
    sec: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            hour: 0,
            min: 0,
            sec: 0.0,
        }
    }
}

impl<'a> FromParser<'a> for Time {
    // Parses a time from "hhmmss.ss";
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let hour = parser.next_n(2)?;
        let min = parser.next_n(2)?;
        let sec = parser.take_until_or_end(',');

        let hour = str::from_utf8(hour)?.parse::<u8>()?;
        let min = str::from_utf8(min)?.parse::<u8>()?;
        let sec = str::from_utf8(sec)?.parse::<f32>()?;

        Ok(Self { hour, min, sec })
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:02}:{:02}:{:02.2}",
            self.hour, self.min, self.sec
        ))
    }
}
