use super::{
    parser::{FromParser, Parser},
    Nmea0183Error,
};

#[derive(Debug, Clone, Copy)]
pub enum FaaMode {
    Autonomous,
    Caution,
    Differential,
    Estimated,
    RtkFloat,
    ManualInput,
    DataNotValid,
    Precise,
    RtkInteger,
    Simulated,
    Unsafe,
}

impl FaaMode {
    pub fn from_char(chr: char) -> Option<Self> {
        Some(match chr {
            'A' => Self::Autonomous,
            'C' => Self::Caution,
            'D' => Self::Differential,
            'E' => Self::Estimated,
            'F' => Self::RtkFloat,
            'M' => Self::ManualInput,
            'N' => Self::DataNotValid,
            'P' => Self::Precise,
            'R' => Self::RtkInteger,
            'S' => Self::Simulated,
            'U' => Self::Unsafe,
            _ => return None,
        })
    }
}

impl<'a> FromParser<'a> for FaaMode {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, Nmea0183Error> {
        let chr = parser.next()?;
        Self::from_char(chr).ok_or(Nmea0183Error::UnexpectedChar(chr))
    }
}
