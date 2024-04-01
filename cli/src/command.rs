use crate::exec::run::exec_run;
use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run(RunArgs),
    Prove(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    // #[arg(short, long)]
    // pub calldata: Option<String>,
    // #[arg(short, long)]
    // pub bytecode: Option<String>,
    // #[arg(short, long)]
    // pub hardcode: Option<String>,
    // #[arg(short, long)]
    // pub file: Option<String>,
    // // #[arg(short, long)]
    // // pub dry_run: bool,
}

pub fn match_operation(cli: &Cli) {
    match &cli.command {
        Commands::Run(_args) => {
            exec_run();
        }
        Commands::Prove(_args) => {
            println!("create proof");
        }
    }
}
