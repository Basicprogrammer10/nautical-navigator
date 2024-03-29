use std::{fmt::Debug, str};

use super::{
    parser::{FromParser, Parser},
    Nmea0183Error,
};

/// `ddmm.mm,d`
#[derive(Clone, Copy)]
pub struct Coordinate {
    pub degree: f32,
}

impl Coordinate {
    pub fn new() -> Self {
        Self { degree: 0.0 }
    }
}

impl<'a> FromParser<'a> for Coordinate {
    // TODO: Fix returning 0.0 when there is no coordinate
    fn parse(parser: &mut Parser<'a>) -> Result<Self, Nmea0183Error> {
        if matches!(parser.peek(), Some(',') | None) {
            parser.skip_if(',');
            return Ok(Self { degree: 0.0 });
        }

        // Take first 2 digits as degrees
        parser.skip_while('0');
        let degrees = parser.next_n(2)?;
        let minutes = parser.take_until(',')?;
        let direction = parser.next()?;

        if !matches!(direction, 'N' | 'S' | 'E' | 'W') {
            return Err(Nmea0183Error::UnexpectedChar(direction));
        }

        let degrees = str::from_utf8(degrees)?.parse::<f32>()?;
        let minutes = str::from_utf8(minutes)?.parse::<f32>()?;
        let negative = direction == 'S' || direction == 'W';

        let degrees = degrees + minutes / 60.0 * if negative { -1.0 } else { 1.0 };
        Ok(Self { degree: degrees })
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deg = self.degree.abs();
        let degrees = deg.floor();
        let minutes = (deg - degrees) * 60.0;
        let seconds = minutes.fract() * 60.0;
        let direction = if self.degree < 0.0 { "S" } else { "N" };
        f.write_fmt(format_args!(
            "{}°{}'{}\"{}",
            degrees,
            minutes.floor(),
            seconds.floor(),
            direction
        ))
    }
}
