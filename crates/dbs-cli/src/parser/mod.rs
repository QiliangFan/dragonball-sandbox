pub mod command;
pub mod args;


use clap::Parser;

use command::CliSub;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: CliSub,
}