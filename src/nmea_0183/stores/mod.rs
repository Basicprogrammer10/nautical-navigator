use crate::misc::delayed::Delayed;

use self::satellites::Satellites;

use super::Sentence;

pub mod satellites;

pub struct Store {
    pub satellites: Satellites,
}

impl Store {
    pub fn new() -> Self {
        Self {
            satellites: Satellites::new(),
        }
    }

    pub fn handle(&mut self, sentence: Sentence) {
        match sentence {
            Sentence::Gsv(gsv) => self.satellites.handle(gsv),
            _ => {}
        }
    }
}
