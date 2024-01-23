use std::{
    io::{BufRead, BufReader},
    time::Duration,
};

mod app;
mod config;
mod misc;
mod nmea_0183;

fn main() {
    let port = serialport::available_ports().expect("No ports found!");
    let port = port.first().unwrap();

    dbg!(&port.port_type);
    let port = serialport::new(&port.port_name, 4800)
        .timeout(Duration::from_secs(2))
        .open()
        .expect("Failed to open port");
    let reader = BufReader::new(port);
    for line in reader.lines().map(|x| x.unwrap()) {
        let msg = nmea_0183::GpsMessage::parse(&line).unwrap();
    }
}
