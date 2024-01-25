use self::{location::Location, satellites::Satellites};

use super::Sentence;

pub mod location;
pub mod satellites;

pub struct Store {
    pub satellites: Satellites,
    pub location: Location,
}

impl Store {
    pub fn new() -> Self {
        Self {
            satellites: Satellites::new(),
            location: Location::new(),
        }
    }

    pub fn handle(&mut self, sentence: Sentence) {
        if let Sentence::Txt(txt) = &sentence {
            println!("[*] GPS MESSAGE: {}", txt.message);
        }

        self.satellites.handle(&sentence);
        self.location.handle(&sentence);
    }
}
