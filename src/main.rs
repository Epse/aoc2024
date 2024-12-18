use clap::{Parser, Subcommand};
mod one;
use one::run as run_one;
mod two;
use two::run as run_two;
mod three;
use three::run as run_three;
mod four;
use four::run as run_four;
mod five;
use five::run as run_five;

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
    Three{},
    Four{},
    Five{},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::One{} => run_one(),
        Commands::Two{} => run_two(cli.debug),
        Commands::Three{} => run_three(),
        Commands::Four{} => run_four(),
        Commands::Five{} => run_five(),
    };
}
