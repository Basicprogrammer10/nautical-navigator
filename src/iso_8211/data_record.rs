use std::io::{Read, Seek};

use super::{error::Iso8211Error, parser::Parser};

#[derive(Debug)]
pub struct DRLedger {
    pub record_length: u32,
    pub field_area_base_address: u32,
    pub field_length_size: u8,
    pub field_position_size: u8,
}

impl DRLedger {
    pub fn parse<T: Read + Seek>(parser: &mut Parser<T>) -> Result<Self, Iso8211Error> {
        let record_length = parser.read_string(5)?.parse::<u32>()?;
        parser.expect_bytes(b" D     ")?;
        let field_area_base_address = parser.read_string(5)?.parse::<u32>()?;
        parser.expect_bytes(b"   ")?;
        let field_length_size = parser.next()? - '0' as u8;
        let field_position_size = parser.next()? - '0' as u8;
        parser.expect_bytes(b"04")?;

        Ok(Self {
            record_length,
            field_area_base_address,
            field_length_size,
            field_position_size,
        })
    }
}
