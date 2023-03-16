//! # borg-connect
//!
//! A binary to run borg commands and report the execution to [borg_vinculum].
#![warn(missing_docs)]

use clap::{Parser, Subcommand};

pub mod config;

/// The available commands for borg-connect
#[derive(Subcommand)]
pub enum Command {
    /// Create a new archive in an existing repository
    Create,
}

/// A helper utility for integrating borg in the vinculum.
#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// The config path of borg-backup
    #[clap(long, default_value_t = String::from("/etc/borg-connect/config.toml"))]
    config_path: String,

    #[clap(subcommand)]
    command: Command,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Create => {}
    }
}
