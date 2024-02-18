use std::io::{BufReader, Read, Seek};

use super::error::Iso8211Error;

pub struct Parser<T> {
    reader: BufReader<T>,
}

impl<T: Read + Seek> Parser<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    pub fn new_buffered(reader: BufReader<T>) -> Self {
        Self { reader }
    }

    pub fn next(&mut self) -> Result<u8, Iso8211Error> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn peek(&mut self) -> Result<u8, Iso8211Error> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        self.reader.seek_relative(-1)?;
        Ok(buf[0])
    }

    pub fn is_eof(&mut self) -> Result<bool, Iso8211Error> {
        let mut buf = [0; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(()) => {
                self.reader.seek_relative(-1)?;
                Ok(false)
            }
            Err(_) => Ok(true),
        }
    }

    pub fn take_until(&mut self, byte: u8) -> Result<Vec<u8>, Iso8211Error> {
        let mut buf = Vec::new();
        loop {
            let b = self.next()?;
            if b == byte {
                break;
            }
            buf.push(b);
        }

        Ok(buf)
    }

    pub fn expect(&mut self, byte: u8) -> Result<(), Iso8211Error> {
        if self.next()? != byte {
            return Err(Iso8211Error::UnexpectedByte(byte));
        }

        Ok(())
    }

    pub fn expect_bytes(&mut self, bytes: &[u8]) -> Result<(), Iso8211Error> {
        for &b in bytes {
            if self.next()? != b {
                return Err(Iso8211Error::UnexpectedByte(b));
            }
        }

        Ok(())
    }
}

impl<T: Read + Seek> Parser<T> {
    pub fn read_string(&mut self, len: usize) -> Result<String, Iso8211Error> {
        let mut buf = vec![0; len];
        self.reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Iso8211Error> {
        let mut buf = [0; N];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}
