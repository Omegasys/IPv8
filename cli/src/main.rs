mod commands;
mod config_loader;

use clap::Parser;
use commands::Cli;

fn main() {
    let cli = Cli::parse();
    commands::execute(cli);
}
