use super::ParseError;

/// In UTC.
pub struct Time {
    hour: u8,
    min: u8,
    /// First value is in seconds, second is 10ths of a second.
    sec: (u8, u8),
}

impl Time {
    /// Parses a time from "hhmmss.ss";
    pub fn parse(time: &str) -> Result<Time, ParseError> {
        if time.len() != 9 {
            return Err(ParseError::IncorrectLength);
        }

        let hour = time[0..2].parse::<u8>()?;
        let min = time[2..4].parse::<u8>()?;
        let sec = time[5..7].parse::<u8>()?;
        let tenths = time[7..9].parse::<u8>()?;

        Ok(Time {
            hour,
            min,
            sec: (sec, tenths),
        })
    }

    pub fn as_secs(&self) -> f32 {
        (self.hour * (60 * 60) + self.min * 60 + self.sec.0) as f32 + self.sec.1 as f32 / 100.0
    }
}
