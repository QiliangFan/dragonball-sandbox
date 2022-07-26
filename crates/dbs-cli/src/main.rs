
pub mod parser;
pub mod vmm;
pub mod utils;

use clap::Parser;
use parser::Cli;

fn main() {
    let config = Cli::parse();
    println!("{:?}", config);
}
