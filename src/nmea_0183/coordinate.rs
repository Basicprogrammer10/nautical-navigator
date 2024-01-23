use super::ParseError;

/// `ddmm.mm``
pub struct Coordinate {
    degree: f32
}

impl Coordinate {
    pub fn parse(cord: &[u8]) -> Result<Coordinate, ParseError> {
        if cord.len() != 6 {
            return Err(ParseError::IncorrectLength)
        }

        // let degree = 

        todo!()
    }
}