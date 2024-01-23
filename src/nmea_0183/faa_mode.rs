use crate::misc::parser::{FromParser, Parser};

use super::ParseError;

#[derive(Debug)]
pub enum FaaMode {
    Autonomous,
    QuectelQuerkCaution,
    Differential,
    Estimated,
    RtkFloat,
    ManualInput,
    DataNotValid,
    Precise,
    RtkInteger,
    Simulated,
    QuectelQuerkUnsafe,
}

impl FaaMode {
    pub fn from_char(chr: char) -> Option<Self> {
        Some(match chr {
            'A' => Self::Autonomous,
            'C' => Self::QuectelQuerkCaution,
            'D' => Self::Differential,
            'E' => Self::Estimated,
            'F' => Self::RtkFloat,
            'M' => Self::ManualInput,
            'N' => Self::DataNotValid,
            'P' => Self::Precise,
            'R' => Self::RtkInteger,
            'S' => Self::Simulated,
            'U' => Self::QuectelQuerkUnsafe,
            _ => return None,
        })
    }
}

impl<'a> FromParser<'a> for FaaMode {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let chr = parser.next()?;
        Self::from_char(chr).ok_or(ParseError::UnexpectedChar(chr))
    }
}
