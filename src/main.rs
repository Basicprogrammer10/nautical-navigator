use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;

mod app;
mod args;
mod commands;
mod config;
mod misc;
mod nmea_0183;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Devices(args) => commands::devices::run(&args)?,
    }

    Ok(())
}

// let port = serialport::new(&port.port_name, 4800)
//     .timeout(Duration::from_secs(2))
//     .open()
//     .expect("Failed to open port");
// let reader = BufReader::new(port);
// for line in reader.lines().map(|x| x.unwrap()) {
//     let msg = nmea_0183::GpsMessage::parse(&line).unwrap();
// }
