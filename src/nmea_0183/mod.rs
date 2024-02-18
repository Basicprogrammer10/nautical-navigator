use std::str;

use packets::geographic_position::GeographicPosition;

use self::{error::Nmea0183Error, packets::{
    active_satellites::ActiveSatellites, ground_speed::GroundSpeed,
    satellites_in_view::SatellitesInView, text::Text,
}};

pub mod coordinate;
pub mod error;
pub mod faa_mode;
pub mod packets;
pub mod parser;
pub mod stores;
pub mod time;

#[derive(Debug)]
pub struct Message {
    /// The two char source of the message.
    /// `GP` is commonly used for a GPS.
    pub identifier: [u8; 2],
    /// The type of message.
    pub message: Sentence,
}

//RMC, GSA, GSV, GLL, VTG
#[derive(Debug)]
pub enum Sentence {
    /// Recommended Minimum Navigation Information.
    Rmc,
    Gsa(ActiveSatellites),
    /// Satellites in view
    Gsv(SatellitesInView),
    /// Geographic Position
    Gll(GeographicPosition),
    /// Track Made Good and Ground Speed.
    Vtg(GroundSpeed),
    /// Text for display.
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

impl Message {
    // TODO: Just use [u8]?
    pub fn parse(bytes: &[u8]) -> Result<Message, Nmea0183Error> {
        if bytes[0] != b'$' {
            return Err(Nmea0183Error::MissingPrefix);
        }

        if bytes.len() < 9 {
            return Err(Nmea0183Error::IncorrectLength);
        }

        let check = &bytes[bytes.len() - 2..];
        let check = u8::from_str_radix(str::from_utf8(check)?, 16)?;
        let last = bytes.len() - 3;
        let calc = checksum(&bytes[0..last]);
        if calc != check {
            return Err(Nmea0183Error::InvalidChecksum);
        }

        let id = [bytes[1], bytes[2]];
        let packet_type = [bytes[3], bytes[4], bytes[5]];

        let to_parse = &bytes[7..last];
        let message = match &packet_type {
            b"GLL" => Sentence::Gll(GeographicPosition::parse(to_parse)?),
            b"GSV" => Sentence::Gsv(SatellitesInView::parse(to_parse)?),
            b"GSA" => Sentence::Gsa(ActiveSatellites::parse(to_parse)?),
            b"VTG" => Sentence::Vtg(GroundSpeed::parse(to_parse)?),
            b"TXT" => Sentence::Txt(Text::parse(to_parse)?),
            _ => return Err(Nmea0183Error::UnknownType(packet_type)),
        };

        Ok(Self {
            identifier: id,
            message,
        })
    }
}
