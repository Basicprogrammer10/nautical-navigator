use std::{
    fmt::Debug,
    io::{Read, Seek},
};

use super::{error::Iso8211Error, parser::Parser, FIELD_TERMINATOR, UNIT_TERMINATOR};

#[derive(Debug)]
pub struct DataDescriptiveRecord {
    pub ledger: DDRLedger,
    // Ends with a field area 0x1E
    pub directory: Vec<DirectoryEntry>,
    // Field Area
}

impl DataDescriptiveRecord {
    pub fn parse<T: Read + Seek>(parser: &mut Parser<T>) -> Result<Self, Iso8211Error> {
        let ledger = DDRLedger::parse(parser)?;
        let directory = DirectoryEntry::parse_all(parser, &ledger)?;

        let control_field = ControlField::parse(parser, &ledger, &directory[0])?;
        dbg!(control_field);

        Ok(Self { ledger, directory })
    }
}

/// Data Descriptive Record.
#[derive(Debug)]
pub struct DDRLedger {
    /// Number of bytes in the record.
    pub record_length: u32,
    /// Number of bytes in ledger and directory.
    /// The base address of the field area.
    pub field_area_base_address: u32,
    /// Size of a field length field (1-9).
    pub field_length_size: u8,
    /// Size of a field position field (1-9).
    pub field_position_size: u8,
}

impl DDRLedger {
    pub fn parse<T: Read + Seek>(parser: &mut Parser<T>) -> Result<Self, Iso8211Error> {
        let record_length = parser.read_string(5)?.parse::<u32>()?;
        parser.expect_bytes(b"3LE1 09")?;
        let field_area_base_address = parser.read_string(5)?.parse::<u32>()?;
        parser.expect_bytes(b" ! ")?;
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

pub struct DirectoryEntry {
    /// The tag of the field.
    pub tag: [u8; 4],
    /// The length of the field.
    pub field_length: u32,
    /// Relative to the start of the field area.
    pub field_position: u32,
}

impl DirectoryEntry {
    pub fn parse_all<T: Read + Seek>(
        parser: &mut Parser<T>,
        ledger: &DDRLedger,
    ) -> Result<Vec<Self>, Iso8211Error> {
        let mut directory = Vec::new();
        while parser.peek()? != FIELD_TERMINATOR {
            let dir_entry = DirectoryEntry::parse(parser, ledger)?;
            directory.push(dir_entry);
        }

        parser.expect(FIELD_TERMINATOR)?;
        Ok(directory)
    }

    pub fn parse<T: Read + Seek>(
        parser: &mut Parser<T>,
        ledger: &DDRLedger,
    ) -> Result<Self, Iso8211Error> {
        let tag = parser.read_bytes()?;
        let field_length = parser
            .read_string(ledger.field_length_size as usize)?
            .parse::<u32>()?;
        let field_position = parser
            .read_string(ledger.field_position_size as usize)?
            .parse::<u32>()?;

        Ok(Self {
            tag,
            field_length,
            field_position,
        })
    }
}

impl Debug for DirectoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = String::from_utf8_lossy(&self.tag);
        f.debug_struct("DirectoryEntry")
            .field("tag", &tag)
            .field("field_length", &self.field_length)
            .field("field_position", &self.field_position)
            .finish()
    }
}

#[derive(Debug)]
pub struct ControlField {
    file_title: String,
    field_tag_pairs: Vec<(String, String)>,
}

impl ControlField {
    pub fn parse<T: Read + Seek>(
        parser: &mut Parser<T>,
        ledger: &DDRLedger,
        entry: &DirectoryEntry,
    ) -> Result<Self, Iso8211Error> {
        parser.expect_bytes(b"0000;&   ")?;
        let file_title = String::from_utf8(parser.take_until(UNIT_TERMINATOR)?)?;

        let mut field_tag_pairs = Vec::new();
        let pairs = (entry.field_length - 11) / 8;
        for _ in 0..pairs {
            let tag = parser.read_string(4)?;
            let value = parser.read_string(4)?;
            field_tag_pairs.push((tag, value));
        }

        Ok(Self {
            file_title,
            field_tag_pairs,
        })
    }
}
