use crate::{error::ParseError, misc::parser::Parser};

#[derive(Debug)]
pub struct Text {
    pub total_sentences: u8,
    pub sentence_number: u8,
    pub text_identifier: u8,
    pub message: String,
}

impl Text {
    pub fn parse(sentence: &[u8]) -> Result<Text, ParseError> {
        let mut parser = Parser::new(sentence).take_on_parse(',');

        let total_sentences = parser.parse::<u8>()?;
        let sentence_number = parser.parse::<u8>()?;
        let text_identifier = parser.parse::<u8>()?;
        let message = parser.parse::<String>()?;

        Ok(Text {
            total_sentences,
            sentence_number,
            text_identifier,
            message,
        })
    }
}
