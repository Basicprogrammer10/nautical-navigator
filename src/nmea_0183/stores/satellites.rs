use std::mem;

use crate::nmea_0183::packets::satellites_in_view::{Satellite, SatellitesInView};

pub struct Satellites {
    /// Number of satellites in view.
    pub in_view: u16,
    /// The satellites in view.
    pub satellites: Vec<Satellite>,
    /// Holds the new satellites until the last sentence is received.
    new_satellites: Vec<Satellite>,
}

impl Satellites {
    pub fn new() -> Self {
        Self {
            in_view: 0,
            satellites: Vec::new(),

            new_satellites: Vec::new(),
        }
    }

    pub fn connected(&self) -> u8 {
        self.satellites.iter().filter(|x| x.snr.is_some()).count() as u8
    }

    pub fn handle(&mut self, sentence: SatellitesInView) {
        if sentence.total_in_group == sentence.sentence_number {
            mem::swap(&mut self.new_satellites, &mut self.satellites);
            self.new_satellites.clear();
        }

        self.in_view = sentence.in_view;
        self.satellites.extend(sentence.satellites.iter().cloned());
    }
}
