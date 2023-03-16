//! # borg-connect
//!
//! A binary to run borg commands and report the execution to [borg_vinculum].
#![warn(missing_docs)]

use std::env;

use clap::{ArgAction, Parser, Subcommand};

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
    /// Specifies the verbosity of the output.
    ///
    /// This option gets overwritten if the environment variable `RUST_LOG` is set.
    #[clap(short = 'v', global = true, action = ArgAction::Count)]
    verbosity: u8,

    /// The config path of borg-backup
    #[clap(long, default_value_t = String::from("/etc/borg-connect/config.toml"))]
    config_path: String,

    #[clap(subcommand)]
    command: Command,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    if env::var("RUST_LOG").is_err() {
        match cli.verbosity {
            0 => env::set_var("RUST_LOG", "info"),
            1 => env::set_var("RUST_LOG", "debug"),
            _ => env::set_var("RUST_LOG", "trace"),
        }
    }

    env_logger::init();

    match cli.command {
        Command::Create => {}
    }
}
