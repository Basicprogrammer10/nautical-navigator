use crate::nmea_0183::ParseError;

pub struct Parser<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, index: 0 }
    }

    pub fn assert_empty(&self) -> Result<(), ParseError> {
        if self.index != self.data.len() {
            return Err(ParseError::RemainingData);
        }

        Ok(())
    }

    pub fn next(&mut self) -> Result<char, ParseError> {
        if self.index >= self.data.len() {
            return Err(ParseError::IncorrectLength);
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
            return Err(ParseError::IncorrectLength);
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
        while self.next().is_ok() && self.peek() != Some(c) {}

        &self.data[start..self.index - 1]
    }

    pub fn remaining_str(&self) -> &'a str {
        std::str::from_utf8(&self.data[self.index..]).unwrap()
    }

    pub fn parse<T: FromParser<'a>>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }
}

pub trait FromParser<'a>: Sized {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError>;
}
