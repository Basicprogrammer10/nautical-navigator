use std::{
    io::{BufRead, BufReader},
    time::Duration,
};

use anyhow::{Context, Result};

use crate::{args::DebugArgs, nmea_0183};

pub fn run(args: &DebugArgs) -> Result<()> {
    let port = serialport::new(&args.device, args.baud_rate)
        .timeout(Duration::from_secs_f32(args.timeout))
        .open()
        .context("Failed to open port")?;

    let mut reader = BufReader::new(port);

    loop {
        let mut line = Vec::new();
        reader.read_until(b'\n', &mut line)?;
        let end = line.len() - 2;

        if args.raw {
            println!("{:?}", String::from_utf8_lossy(&line[..end]));
        }

        let msg = nmea_0183::GpsMessage::parse(&line[..end]);
        match msg {
            Ok(msg) => println!("{:?}", msg),
            Err(err) => {
                if !args.ignore_errors {
                    return Err(err.into());
                } else {
                    eprintln!("Error: {:?}", err);
                }
            }
        }
    }
}
