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

    let reader = BufReader::new(port);
    for line in reader.lines() {
        let line = line.context("Failed to read line")?;

        if args.raw {
            println!("{}", line);
        }

        let msg = nmea_0183::GpsMessage::parse(&line)?;
        println!("{:?}", msg);
    }

    Ok(())
}
