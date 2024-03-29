use std::{collections::VecDeque, mem};

use crate::{
    consts::HISTORY_SAMPLES,
    misc::average::Average,
    nmea_0183::{
        packets::satellites_in_view::{Satellite, SatellitesInView},
        Sentence,
    },
};

pub struct Satellites {
    /// Number of satellites in view.
    pub in_view: u16,
    /// The satellites in view.
    pub satellites: Vec<Satellite>,
    pub avg_sdr_history: VecDeque<f32>,

    /// Holds the new satellites until the last sentence is received.
    new_satellites: Vec<Satellite>,
}

impl Satellites {
    pub fn new() -> Self {
        Self {
            in_view: 0,
            satellites: Vec::new(),
            avg_sdr_history: VecDeque::new(),
            new_satellites: Vec::new(),
        }
    }

    pub fn connected(&self) -> u8 {
        self.satellites.iter().filter(|x| x.snr.is_some()).count() as u8
    }

    pub fn handle(&mut self, sentence: &Sentence) {
        if let Sentence::Gsv(sentence) = sentence {
            self.handle_inner(sentence);
        }
    }

    fn handle_inner(&mut self, sentence: &SatellitesInView) {
        self.in_view = sentence.in_view;
        self.new_satellites
            .extend(sentence.satellites.iter().cloned());

        if sentence.total_in_group == sentence.sentence_number {
            mem::swap(&mut self.new_satellites, &mut self.satellites);
            self.new_satellites.clear();

            let avg = self
                .satellites
                .iter()
                .flat_map(|x| x.snr)
                .map(|x| x as f32)
                .average();
            self.avg_sdr_history.push_back(avg);

            while self.avg_sdr_history.len() > HISTORY_SAMPLES {
                self.avg_sdr_history.pop_front();
            }
        }
    }
}
