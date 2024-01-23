use std::num;

use thiserror::Error;

pub mod coordinate;
pub mod time;

#[derive(Debug)]
pub struct GpsMessage {
    /// The two char source of the message.
    /// `GP` is commonly used for a GPS.
    identifier: [u8; 2],
}

//RMC, GSA, GSV, GLL, VTG
pub enum Type {
    /// Recommended Minimum Navigation Information.
    Rmc,
    Gsa,
    Gsv,
    /// Geographic Position
    Gll,
    Vtg,
}

fn checksum(sentence: &[u8]) -> u8 {
    let mut out = 0;
    for byte in sentence {
        if matches!(byte, b'$' | b'I' | b'*') {
            continue;
        }

        out ^= byte;
    }
    out
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The message is missing the `$` prefix")]
    MissingPrefix,
    #[error("The length of the input was not long enough")]
    IncorrectLength,
    #[error("The message's checksum did not match")]
    InvalidChecksum,
    #[error("An integer was invalid")]
    InvalidNumber(#[from] num::ParseIntError),
}

impl GpsMessage {
    // TODO: Just use [u8]?
    pub fn parse(sentence: &str) -> Result<GpsMessage, ParseError> {
        let bytes = sentence.as_bytes();
        if bytes[0] != b'$' {
            return Err(ParseError::MissingPrefix);
        }

        if sentence.len() < 9 {
            return Err(ParseError::IncorrectLength);
        }

        let check = &sentence[sentence.len() - 2..];
        let check = u8::from_str_radix(check, 16)?;
        let calc = checksum(&bytes[0..bytes.len() - 3]);
        if calc != check {
            return Err(ParseError::InvalidChecksum);
        }

        let id = [bytes[1], bytes[2]];
        let packet_type = [bytes[3], bytes[4], bytes[5]];

        println!("{packet_type:?}");
        Ok(Self { identifier: id })
    }
}
