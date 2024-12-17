use clap::{Parser, Subcommand};
mod one;
use one::run as run_one;
mod two;
use two::run as run_two;

mod parse;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, action)]
    debug: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    One{},
    Two{},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::One{} => run_one(),
        Commands::Two{} => run_two(cli.debug),
    };
}
