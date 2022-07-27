pub mod command;
pub mod args;


use clap::Parser;

pub use command::CliSub;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CliSub,
}