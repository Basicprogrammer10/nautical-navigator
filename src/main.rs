use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;

mod app;
mod args;
mod commands;
mod consts;
mod iso_8211;
mod log;
mod misc;
mod nmea_0183;

fn main() -> Result<()> {
    let enc = ::std::fs::File::open(r"V:\Downloads\EncMaps\US\US556660\US556660.000").unwrap();
    let mut parser = iso_8211::parser::Parser::new(enc);
    let ddr = iso_8211::data_descriptive_record::DataDescriptiveRecord::parse(&mut parser)?;
    dbg!(ddr);

    return Ok(());

    let args = Args::parse();

    match args.subcommand {
        SubCommand::Run(args) => commands::run::run(&args)?,
        SubCommand::Devices(args) => commands::devices::run(&args)?,
        SubCommand::Debug(args) => commands::debug::run(&args)?,
    }

    Ok(())
}
