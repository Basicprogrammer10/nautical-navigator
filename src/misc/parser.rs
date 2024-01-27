use std::str;

use crate::error::ParseError;

pub struct Parser<'a> {
    data: &'a [u8],
    index: usize,

    take_on_parse: Option<char>,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
            take_on_parse: None,
        }
    }

    pub fn take_on_parse(self, c: char) -> Self {
        Self {
            take_on_parse: Some(c),
            ..self
        }
    }

    pub fn take_while(&mut self, c: fn(char) -> bool) -> &'a [u8] {
        let start = self.index;
        while self.peek().map_or(false, c) {
            self.index += 1;
        }

        &self.data[start..self.index]
    }

    pub fn assert_empty(&self) -> Result<(), ParseError> {
        if self.index != self.data.len() {
            return Err(ParseError::RemainingData);
        }

        Ok(())
    }

    pub fn next(&mut self) -> Result<char, ParseError> {
        if self.index >= self.data.len() {
            return Err(ParseError::Incomplete);
        }

        let out = self.data[self.index] as char;
        self.index += 1;
        Ok(out)
    }

    pub fn peek(&self) -> Option<char> {
        if self.index >= self.data.len() {
            return None;
        }

        Some(self.data[self.index] as char)
    }

    pub fn next_n(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        if self.index + n >= self.data.len() {
            return Err(ParseError::Incomplete);
        }

        let out = &self.data[self.index..self.index + n];
        self.index += n;
        Ok(out)
    }

    pub fn skip(&mut self, n: usize) {
        self.index += n;
    }

    pub fn expect(&mut self, c: char) -> Result<(), ParseError> {
        if self.next()? != c {
            return Err(ParseError::UnexpectedChar(c));
        }

        Ok(())
    }

    pub fn expect_bytes(&mut self, bytes: &[u8]) -> Result<(), ParseError> {
        let mut i = 0;
        while i < bytes.len() {
            if self.next()? != bytes[i] as char {
                return Err(ParseError::UnexpectedChar(bytes[i] as char));
            }

            i += 1;
        }

        Ok(())
    }

    pub fn skip_if(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.index += 1;
            return true;
        }

        false
    }

    pub fn skip_while(&mut self, c: char) {
        while self.skip_if(c) {}
    }

    pub fn take_until(&mut self, c: char) -> Result<&'a [u8], ParseError> {
        let start = self.index;
        while self.next()? != c {}

        Ok(&self.data[start..self.index - 1])
    }

    pub fn take_until_or_end(&mut self, c: char) -> &'a [u8] {
        let start = self.index;
        while self.peek() != Some(c) && self.next().is_ok() {}

        &self.data[start..self.index]
    }

    pub fn remaining_str(&self) -> &'a str {
        std::str::from_utf8(&self.data[self.index..]).unwrap()
    }

    pub fn parse<T: FromParser<'a>>(&mut self) -> Result<T, ParseError> {
        let res = T::parse(self);
        if let Some(c) = self.take_on_parse {
            let _ = self.expect(c);
        }
        res
    }
}

pub trait FromParser<'a>: Sized {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError>;
}

impl<'a> FromParser<'a> for u8 {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let bytes = parser.take_while(|c| c.is_ascii_digit());
        Ok(str::from_utf8(bytes)?.parse::<u8>()?)
    }
}

impl<'a> FromParser<'a> for i8 {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let bytes = parser.take_while(|c| matches!(c, '-' | '+' | '0'..='9'));
        Ok(str::from_utf8(bytes)?.parse::<i8>()?)
    }
}

impl<'a> FromParser<'a> for u16 {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let bytes = parser.take_while(|c| c.is_ascii_digit());
        Ok(str::from_utf8(bytes)?.parse::<u16>()?)
    }
}

impl<'a> FromParser<'a> for f32 {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let bytes = parser.take_while(|c| matches!(c, '.' | '-' | '+' | 'e' | 'E' | '0'..='9'));
        Ok(str::from_utf8(bytes)?.parse::<f32>()?)
    }
}

impl<'a> FromParser<'a> for String {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
        let bytes = parser.take_until_or_end(',');

        // Handle escape codes (^(ascii hex))
        let mut i = 0;
        let mut out = String::with_capacity(bytes.len());

        while i < bytes.len() {
            let c = bytes[i];
            if c == b'^' {
                let hex = &bytes[i + 1..i + 3];
                let hex = u8::from_str_radix(str::from_utf8(hex)?, 16)?;
                out.push(hex as char);
                i += 3;
            } else {
                out.push(c as char);
                i += 1;
            }
        }

        Ok(out)
    }
}

#[macro_export]
macro_rules! quick_parser {
    ($for:ty, {
        $($chr:literal => $variant:ident),*$(,)?
    }) => {
        impl<'a> FromParser<'a> for $for {
            fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError> {
                let chr = parser.next()?;
                Ok(match chr {
                    $($chr => Self::$variant),*,
                    _ => return Err(ParseError::UnexpectedChar(chr)),
                })
            }
        }
    };
}
