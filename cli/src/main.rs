use clap::Parser;
use command::Cli;

pub mod command;
mod exec;

fn main() {
    let cli = Cli::parse();
    command::match_operation(&cli);
}
