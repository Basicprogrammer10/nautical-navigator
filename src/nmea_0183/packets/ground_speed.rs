use crate::{error::ParseError, misc::parser::Parser, nmea_0183::faa_mode::FaaMode};

#[derive(Debug)]
pub struct GroundSpeed {
    /// Course over ground, true.
    course_true: Option<f32>,
    /// Course over ground, magnetic.
    course_magnetic: Option<f32>,
    /// Speed over ground, knots.
    speed_knots: f32,
    /// Speed over ground, kilometers per hour.
    speed_kph: f32,
    /// FAA mode.
    faa_mode: FaaMode,
}

impl GroundSpeed {
    pub fn parse(sentence: &[u8]) -> Result<GroundSpeed, ParseError> {
        let mut parser = Parser::new(sentence).take_on_parse(',');
        let course_true = parser.parse::<f32>().ok();
        parser.expect_bytes(b"T,")?;
        let course_magnetic = parser.parse::<f32>().ok();
        parser.expect_bytes(b"M,")?;
        let speed_knots = parser.parse::<f32>()?;
        parser.expect_bytes(b"N,")?;
        let speed_kph = parser.parse::<f32>()?;
        parser.expect_bytes(b"K,")?;
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
