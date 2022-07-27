
pub mod parser;
pub mod vmm;
pub mod utils;

use clap::Parser;
use parser::{Cli, CliSub};
use crate::utils::info::InstanceInfo;
use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    let cli = Cli::parse();



    match &cli.command {
        CliSub::Create(create_args) =>  {

        }
        CliSub::Config(config_args) => {

        }
        _ => {
            // Nothing to do, error handled by clap:Parser.
        }
    }

}
