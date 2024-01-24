use crate::{
    misc::parser::{FromParser, Parser},
    quick_parser,
};

use super::super::{coordinate::Coordinate, faa_mode::FaaMode, time::Time, ParseError};

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
        let mut parser = Parser::new(sentence).take_on_parse(',');
        let latitude = parser.parse::<Coordinate>()?;
        let longitude = parser.parse::<Coordinate>()?;
        let time = parser.parse::<Time>()?;
        let status = parser.parse::<Status>()?;
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

quick_parser!(Status, {
    'V' => DataInvalid,
    'A' => DataValid,
});
