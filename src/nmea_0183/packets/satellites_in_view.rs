use crate::nmea_0183::{
    error::Nmea0183Error,
    parser::{FromParser, Parser},
};

#[derive(Debug)]
pub struct SatellitesInView {
    /// Total number of messages of this type in this cycle.
    pub total_in_group: u8,
    /// The index of this message type in the cycle.
    pub sentence_number: u8,
    /// Total number of satellites in view.
    pub in_view: u16,
    /// The satellites contained in this message (total_in_group).
    pub satellites: Box<[Satellite]>,
}

#[derive(Debug, Clone)]
pub struct Satellite {
    /// The id of this satellite.
    pub id: u8,
    /// Elevation in degrees, +/- 90.
    pub elevation: Option<i8>,
    /// Azimuth, degrees from true north, 000 to 359.
    pub azimuth: Option<u16>,
    /// Signal to noise ratio, 00-99 dB.
    pub snr: Option<u8>,
}

impl SatellitesInView {
    pub fn parse(sentence: &[u8]) -> Result<SatellitesInView, Nmea0183Error> {
        let mut parser = Parser::new(sentence).take_on_parse(',');

        let total_in_group = parser.parse::<u8>()?;
        let sentence_number = parser.parse::<u8>()?;
        let in_view = parser.parse::<u16>()?;

        let mut satellites = Vec::new();
        for _ in 0..4 {
            match parser.parse::<Satellite>() {
                Ok(satellite) => satellites.push(satellite),
                Err(Nmea0183Error::Incomplete) => break,
                Err(err) => return Err(err),
            }
        }

        let satellites = satellites.into_boxed_slice();

        Ok(SatellitesInView {
            total_in_group,
            sentence_number,
            in_view,
            satellites,
        })
    }
}

impl<'a> FromParser<'a> for Satellite {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, Nmea0183Error> {
        let id = parser.parse::<u8>().ok().ok_or(Nmea0183Error::Incomplete)?;
        let elevation = parser.parse::<i8>().ok();
        let azimuth = parser.parse::<u16>().ok();
        let snr = parser.parse::<u8>().ok();

        Ok(Satellite {
            id,
            elevation,
            azimuth,
            snr,
        })
    }
}
