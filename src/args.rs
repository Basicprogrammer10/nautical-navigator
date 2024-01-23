use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// List serial devices available
    Devices(DevicesArgs),
    /// Prints decoded NMEA 0183 messages from a serial device
    Debug(DebugArgs),
}

#[derive(Parser, Debug)]
pub struct DevicesArgs {}

#[derive(Parser, Debug)]
pub struct DebugArgs {
    /// Serial device to read from
    pub device: String,
    /// Baud rate to use
    #[clap(short, long, default_value = "4800")]
    pub baud_rate: u32,
    /// Time to wait for a message before timing out (in seconds)
    #[clap(short, long, default_value = "2")]
    pub timeout: f32,
    /// Print raw NMEA 0183 messages
    #[clap(short, long)]
    pub raw: bool,
    /// Ignore message parsing errors
    #[clap(short, long)]
    pub ignore_errors: bool,
}
