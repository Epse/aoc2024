use clap::{Parser, Subcommand};
mod one;
use one::run as run_one;

mod parse;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    One{}
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::One{} => run_one(),
    };
}
