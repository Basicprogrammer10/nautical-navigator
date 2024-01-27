use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;

mod app;
mod args;
mod commands;
mod consts;
mod error;
mod misc;
mod nmea_0183;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Run(args) => commands::run::run(&args)?,
        SubCommand::Devices(args) => commands::devices::run(&args)?,
        SubCommand::Debug(args) => commands::debug::run(&args)?,
    }

    Ok(())
}
