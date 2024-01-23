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
}

#[derive(Parser, Debug)]
pub struct DevicesArgs {}
