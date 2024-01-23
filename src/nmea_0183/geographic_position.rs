use crate::misc::parser::{FromParser, Parser};

use super::{coordinate::Coordinate, faa_mode::FaaMode, time::Time, ParseError};

/// `ddmm.mm,a,dddmm.mm,a,hhmmss.ss,a,m
#[derive(Debug)]
pub struct GeographicPosition {
    latitude: Coordinate,
    longitude: Coordinate,
    time: Time,
    status: Status,
    mode: FaaMode,
}

#[derive(Debug)]
pub enum Status {
    DataValid,
    DataInvalid,
}

impl GeographicPosition {
    pub fn parse(sentence: &[u8]) -> Result<GeographicPosition, ParseError> {
        let mut parser = Parser::new(sentence);
        let latitude = parser.parse::<Coordinate>()?;
        parser.expect(',')?;
        let longitude = parser.parse::<Coordinate>()?;
        parser.expect(',')?;
        let time = parser.parse::<Time>()?;
        parser.expect(',')?;
        let status = parser.parse::<Status>()?;
        parser.expect(',')?;
        let mode = parser.parse::<FaaMode>()?;

        parser.assert_empty()?;

        Ok(GeographicPosition {
            latitude,
            longitude,
            time,
            status,
            mode,
        })
    }
}

impl<'a> FromParser<'a> for Status {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let chr = parser.next()?;
        Ok(match chr {
            'A' => Self::DataValid,
            'V' => Self::DataInvalid,
            _ => return Err(ParseError::UnexpectedChar(chr)),
        })
    }
}
