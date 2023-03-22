//! # borg-drone
//!
//! A binary to run borg commands and report the execution to [borg_vinculum].
#![warn(missing_docs)]

use std::env;

use clap::{ArgAction, Parser, Subcommand};

use crate::commands::CheckOptions;
use crate::config::get_config;

pub mod commands;
pub mod config;

/// The available commands for borg-connect
#[derive(Subcommand)]
pub enum Command {
    /// Check the state of an repository, archive or both
    Check {
        /// Specify an alternative path on the remote path
        #[clap(long)]
        remote_path: Option<String>,
        /// The path of the remote repository
        repository_path: String,
    },
    /// Create a new archive in an existing repository
    Create {
        /// Start in UI mode
        #[clap(long, default_value_t = false)]
        ui: bool,
    },
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
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    if env::var("RUST_LOG").is_err() {
        match cli.verbosity {
            0 => env::set_var("RUST_LOG", "info"),
            1 => env::set_var("RUST_LOG", "debug"),
            _ => env::set_var("RUST_LOG", "trace"),
        }
    }

    env_logger::init();

    let config = get_config(&cli.config_path)?;

    match cli.command {
        Command::Check {
            remote_path,
            repository_path,
        } => {
            let remote_path = remote_path.or(config.borg.remote_path);

            let c = CheckOptions {
                repository_path,
                remote_path,
            };

            commands::run_check(c).await?;
        }
        Command::Create { ui } => {}
    }

    Ok(())
}
