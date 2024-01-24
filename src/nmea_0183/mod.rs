use std::str;

use crate::error::ParseError;

use packets::geographic_position::GeographicPosition;

use self::packets::{active_satellites::ActiveSatellites, text::Text};

pub mod coordinate;
pub mod faa_mode;
pub mod packets;
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
    Gsa(ActiveSatellites),
    Gsv,
    /// Geographic Position
    Gll(GeographicPosition),
    Vtg,
    Txt(Text),
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

        let to_parse = &bytes[7..last];
        let message = match &packet_type {
            b"GLL" => Type::Gll(GeographicPosition::parse(to_parse)?),
            b"GSA" => Type::Gsa(ActiveSatellites::parse(to_parse)?),
            b"TXT" => Type::Txt(Text::parse(to_parse)?),
            _ => return Err(ParseError::UnknownType(packet_type)),
        };

        Ok(Self {
            identifier: id,
            message,
        })
    }
}
