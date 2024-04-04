use std::{fs::File, io::BufReader};

use crate::exec::run::exec_run;
use circuits::prove;
use clap::{command, Args, Parser, Subcommand};
use runtime::trace::Step;

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
    #[arg(short, long)]
    pub trace: Option<String>,
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
        Commands::Prove(args) => {
            println!("create proof");
            let trace = args.trace.as_deref();
            let steps = get_trace_from_file(trace.unwrap());
            prove(steps);
        }
    }
}

fn get_trace_from_file(path: &str) -> Vec<Step> {
    let file = File::open(path).expect("open file");
    let reader = BufReader::new(file);
    let trace = serde_json::from_reader(reader).expect("read json");
    trace
}
