use clap::{Parser, Subcommand};
use crate::config_loader::load_config;

#[derive(Parser)]
#[command(name = "ipv8")]
#[command(about = "IPv8 Network CLI Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run a network simulation
    Simulate {
        #[arg(short, long)]
        nodes: usize,
    },

    /// Inspect a packet
    Inspect {
        #[arg(short, long)]
        data: String,
    },

    /// Generate traffic
    Traffic {
        #[arg(short, long)]
        count: usize,
    },

    /// Load configuration
    Config {
        #[arg(short, long)]
        file: String,
    },
}

pub fn execute(cli: Cli) {
    match cli.command {
        Command::Simulate { nodes } => {
            println!("Starting IPv8 simulation with {} nodes", nodes);
        }

        Command::Inspect { data } => {
            println!("Inspecting packet: {}", data);
        }

        Command::Traffic { count } => {
            println!("Generating {} packets of traffic", count);
        }

        Command::Config { file } => {
            let cfg = load_config(&file);
            println!("Loaded config: {:?}", cfg);
        }
    }
}
