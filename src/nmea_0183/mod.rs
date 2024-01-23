use std::num;
use std::str;

use anyhow::Context;
use thiserror::Error;

use self::geographic_position::GeographicPosition;

pub mod coordinate;
pub mod faa_mode;
pub mod geographic_position;
pub mod time;

#[derive(Debug)]
pub struct GpsMessage {
    /// The two char source of the message.
    /// `GP` is commonly used for a GPS.
    identifier: [u8; 2],
    /// The type of message.
    message: Type,
}

//RMC, GSA, GSV, GLL, VTG
#[derive(Debug)]
pub enum Type {
    /// Recommended Minimum Navigation Information.
    Rmc,
    Gsa,
    Gsv,
    /// Geographic Position
    Gll(GeographicPosition),
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
    #[error("A float was invalid")]
    InvalidFloat(#[from] num::ParseFloatError),
    #[error("Unknown message type")]
    UnknownType([u8; 3]),
    #[error("Non UTF-8 character")]
    NonUtf8(#[from] std::str::Utf8Error),
    #[error("Unexpected character")]
    UnexpectedChar(char),
    #[error("Parser has remaining data")]
    RemainingData,
}

impl GpsMessage {
    // TODO: Just use [u8]?
    pub fn parse(bytes: &[u8]) -> Result<GpsMessage, ParseError> {
        if bytes[0] != b'$' {
            return Err(ParseError::MissingPrefix);
        }

        if bytes.len() < 9 {
            return Err(ParseError::IncorrectLength);
        }

        let check = &bytes[bytes.len() - 2..];
        let check = u8::from_str_radix(str::from_utf8(check)?, 16)?;
        let last = bytes.len() - 3;
        let calc = checksum(&bytes[0..last]);
        if calc != check {
            return Err(ParseError::InvalidChecksum);
        }

        let id = [bytes[1], bytes[2]];
        let packet_type = [bytes[3], bytes[4], bytes[5]];

        let message = match &packet_type {
            b"GLL" => Type::Gll(GeographicPosition::parse(&bytes[7..last])?),
            _ => return Err(ParseError::UnknownType(packet_type)),
        };

        Ok(Self {
            identifier: id,
            message,
        })
    }
}
