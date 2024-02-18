use crate::{
    nmea_0183::{
        coordinate::Coordinate, error::Nmea0183Error, faa_mode::FaaMode, parser::Parser, time::Time,
    },
    quick_parser,
};

/// `ddmm.mm,a,dddmm.mm,a,hhmmss.ss,a,m
#[derive(Debug)]
pub struct GeographicPosition {
    pub latitude: Coordinate,
    pub longitude: Coordinate,
    pub time: Time,
    pub status: Status,
    pub mode: FaaMode,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    DataValid,
    DataInvalid,
}

impl GeographicPosition {
    pub fn parse(sentence: &[u8]) -> Result<GeographicPosition, Nmea0183Error> {
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
