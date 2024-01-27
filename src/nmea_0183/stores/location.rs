use crate::nmea_0183::{
    coordinate::Coordinate,
    packets::{
        active_satellites::{ActiveSatellites, Fix},
        geographic_position::{GeographicPosition, Status},
    },
    time::Time,
    Sentence,
};

pub struct Location {
    pub latitude: Coordinate,
    pub longitude: Coordinate,
    pub time: Time,
    pub status: Status,
    pub fix: Fix,
    pub pdop: f32,
    pub hdop: f32,
    pub vdop: f32,
}

impl Location {
    pub fn new() -> Self {
        Self {
            latitude: Coordinate::new(),
            longitude: Coordinate::new(),
            time: Time::new(),
            status: Status::DataInvalid,
            fix: Fix::NoFix,
            pdop: 0.0,
            hdop: 0.0,
            vdop: 0.0,
        }
    }

    pub fn handle(&mut self, sentence: &Sentence) {
        match sentence {
            Sentence::Gll(sentence) => self.handle_pos_inner(sentence),
            Sentence::Gsa(sentence) => self.handel_active_satellites(sentence),
            _ => {}
        }
    }

    fn handle_pos_inner(&mut self, sentence: &GeographicPosition) {
        self.latitude = sentence.latitude;
        self.longitude = sentence.longitude;
        self.time = sentence.time;
        self.status = sentence.status;
    }

    fn handel_active_satellites(&mut self, sentence: &ActiveSatellites) {
        self.fix = sentence.fix;
        self.pdop = sentence.pdop;
        self.hdop = sentence.hdop;
        self.vdop = sentence.vdop;
    }
}
