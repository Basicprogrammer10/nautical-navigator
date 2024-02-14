use std::{
    io::{BufRead, BufReader},
    str,
    sync::Arc,
    thread,
    time::Duration,
};

use anyhow::Result;
use eframe::NativeOptions;
use parking_lot::Mutex;

use crate::{
    app::App,
    args::RunArgs,
    error::ParseError,
    log::Log,
    nmea_0183::{self, stores::Store},
};

pub fn run(args: &RunArgs) -> Result<()> {
    let log = Log::new();
    let store = Arc::new(Mutex::new(Store::new(log.clone())));
    let app = App::new(args.clone(), store.clone(), log.clone());

    let serial = serialport::new(args.device.as_str(), args.baud_rate)
        .timeout(Duration::from_secs_f32(args.timeout))
        .open()?;
    let mut reader = BufReader::new(serial);

    thread::spawn(move || loop {
        let mut line = Vec::new();
        reader.read_until(b'\n', &mut line).unwrap();
        let end = line.len() - 2;

        let msg = nmea_0183::Message::parse(&line[..end]);
        match msg {
            Ok(msg) => store.lock().handle(msg.message),
            Err(ParseError::UnknownType(..)) => {}
            Err(err) => {
                log.warning(format!("NMEA Error: {:?}", err));
                eprintln!(
                    "[-] NMEA Error: {:?}\n |  {}",
                    err,
                    str::from_utf8(&line[..end]).unwrap()
                )
            }
        }
    });

    eframe::run_native(
        "Nautical Navigator",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(app)),
    )
    .unwrap();

    Ok(())
}
