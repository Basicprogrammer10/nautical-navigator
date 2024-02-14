use crate::log::Log;

use self::{location::Location, satellites::Satellites};

use super::Sentence;

pub mod location;
pub mod satellites;

pub struct Store {
    log: Log,
    pub satellites: Satellites,
    pub location: Location,
}

impl Store {
    pub fn new(log: Log) -> Self {
        Self {
            log,
            satellites: Satellites::new(),
            location: Location::new(),
        }
    }

    pub fn handle(&mut self, sentence: Sentence) {
        if let Sentence::Txt(txt) = &sentence {
            self.log.info(format!("GPS MESSAGE: {}", txt.message));
            println!("[*] GPS MESSAGE: {}", txt.message);
        }

        self.satellites.handle(&sentence);
        self.location.handle(&sentence);
    }
}
