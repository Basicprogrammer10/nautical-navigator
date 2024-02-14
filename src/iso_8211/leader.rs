use std::str;

use crate::error::ParseError;

/// Data Descriptive Record.
struct DDRLedger {
    record_length: u32,
    field_area_base_address: u32,
    field_length_size: u8,
    field_position_size: u8,
    field_tag_size: u8,
}

impl DDRLedger {
    pub fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        if bytes.len() != 24 {
            return Err(ParseError::IncorrectLength);
        }

        assert_eq!(bytes[5], b'3');
        assert_eq!(bytes[6], b'L');
        assert_eq!(bytes[7], b'E');
        assert_eq!(bytes[8], b'1');
        assert_eq!(bytes[9], b'0');
        assert_eq!(&bytes[10..12], b"09");
        assert_eq!(&bytes[17..20], b" ! ");
        assert_eq!(bytes[22], b'0');
        assert_eq!(bytes[23], b'4');

        Ok(Self {
            record_length: str::from_utf8(&bytes[0..5])?.parse::<u32>()?,
            field_area_base_address: str::from_utf8(&bytes[12..17])?.parse::<u32>()?,
            field_length_size: bytes[17],
            field_position_size: bytes[18],
            field_tag_size: bytes[20],
        })
    }
}
