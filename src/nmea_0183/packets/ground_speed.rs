use crate::nmea_0183::{error::Nmea0183Error, faa_mode::FaaMode, parser::Parser};

#[derive(Debug)]
pub struct GroundSpeed {
    /// Course over ground, true.
    course_true: Option<f32>,
    /// Course over ground, magnetic.
    course_magnetic: Option<f32>,
    /// Speed over ground, knots.
    speed_knots: Option<f32>,
    /// Speed over ground, kilometers per hour.
    speed_kph: Option<f32>,
    /// FAA mode.
    faa_mode: FaaMode,
}

impl GroundSpeed {
    pub fn parse(sentence: &[u8]) -> Result<GroundSpeed, Nmea0183Error> {
        let mut parser = Parser::new(sentence).take_on_parse(',');
        let course_true = parser.parse::<f32>().ok();
        parser.skip_if('T');
        parser.expect(',')?;
        let course_magnetic = parser.parse::<f32>().ok();
        parser.skip_if('M');
        parser.expect(',')?;
        let speed_knots = parser.parse::<f32>().ok();
        parser.skip_if('N');
        parser.expect(',')?;
        let speed_kph = parser.parse::<f32>().ok();
        parser.skip_if('K');
        parser.expect(',')?;
        let faa_mode = parser.parse::<FaaMode>()?;
        parser.assert_empty()?;

        Ok(GroundSpeed {
            course_true,
            course_magnetic,
            speed_knots,
            speed_kph,
            faa_mode,
        })
    }
}
