use std::str;

use super::error::Iso8211Error;

/// Data Descriptive Record.
#[derive(Debug)]
pub struct DDRLedger {
    pub record_length: u32,
    pub field_area_base_address: u32,
    pub field_length_size: u8,
    pub field_position_size: u8,
}

pub struct DRLedger {
    pub record_length: u32,
    pub field_area_base_address: u32,
    pub field_length_size: u8,
    pub field_position_size: u8,
}

impl DDRLedger {
    pub fn parse(bytes: &[u8]) -> Result<Self, Iso8211Error> {
        if bytes.len() != 24 {
            return Err(Iso8211Error::IncorrectLength);
        }

        assert_eq!(bytes[5], b'3');
        assert_eq!(bytes[6], b'L');
        assert_eq!(bytes[7], b'E');
        assert_eq!(bytes[8], b'1');
        assert_eq!(bytes[9], b' ');
        assert_eq!(&bytes[10..12], b"09");
        assert_eq!(&bytes[17..20], b" ! ");
        assert_eq!(bytes[22], b'0');
        assert_eq!(bytes[23], b'4');

        Ok(Self {
            record_length: str::from_utf8(&bytes[0..5])?.parse::<u32>()?,
            field_area_base_address: str::from_utf8(&bytes[12..17])?.parse::<u32>()?,
            field_length_size: bytes[20],
            field_position_size: bytes[21],
        })
    }
}

impl DRLedger {
    pub fn parse(bytes: &[u8]) -> Result<Self, Iso8211Error> {
        if bytes.len() != 24 {
            return Err(Iso8211Error::IncorrectLength);
        }

        assert_eq!(&bytes[5..12], b" D     ");
        assert_eq!(&bytes[17..20], b"   ");
        assert_eq!(&bytes[22..24], b"04");

        Ok(Self {
            record_length: str::from_utf8(&bytes[0..5])?.parse::<u32>()?,
            field_area_base_address: str::from_utf8(&bytes[12..17])?.parse::<u32>()?,
            field_length_size: bytes[20],
            field_position_size: bytes[21],
        })
    }
}
