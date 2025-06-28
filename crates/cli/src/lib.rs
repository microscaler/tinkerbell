//! CLI library exposing commands and transport utilities.

pub mod commands;
pub mod transport;

use clap::{Parser, Subcommand};

/// Global command line options and subcommands.
#[derive(Parser, Debug)]
#[command(name = "tctl")]
pub struct Cli {
    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,
    /// Output in plain text
    #[arg(long, global = true)]
    pub plain: bool,
    /// Endpoint of the daemon
    #[arg(long, default_value = "unix:/tmp/tinkerbell.sock", global = true)]
    pub endpoint: String,
    #[command(subcommand)]
    pub command: Commands,
}

/// CLI subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show daemon status
    Status,
}
