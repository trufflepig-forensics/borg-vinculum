//! # borg-drone
//!
//! A binary to run borg commands and report the execution to [borg_vinculum].
#![warn(missing_docs)]

use std::env;

use clap::{ArgAction, Parser, Subcommand};
use log::{debug, info, warn};

use crate::api::Api;
use crate::config::get_config;
use crate::create::run_create;
use crate::hooks::{run_post_hook, run_pre_hook};

pub mod api;
pub mod config;
pub mod create;
pub mod hooks;

/// The available commands for borg-connect
#[derive(Subcommand)]
pub enum Command {
    /// Create a new archive in an existing repository
    Create {
        /// Run the backup as dry run.
        ///
        /// This will execute the pre and post hooks, but will skip the creation of the backup.
        #[clap(long, default_value_t = false)]
        dry_run: bool,

        /// Output the progress while archive creation
        #[clap(short = 'p', long, default_value_t = false)]
        progress: bool,
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
    #[clap(long, default_value_t = String::from("/etc/borg-drone/config.toml"))]
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
        Command::Create { dry_run, progress } => {
            debug!("Initializing API");
            let api = Api::new(config.vinculum_address.clone(), &config.vinculum_token)?;

            if config.pre_hook.is_empty() {
                info!("Skipping pre hook");
            } else {
                info!("Starting pre hook");
                run_pre_hook(&api, &config).await?;
                info!("Finished pre hook");
            }

            if dry_run {
                info!("Skipping archive creation");
            } else {
                info!("Starting archive creation");
                run_create(&api, &config, progress).await?;
                info!("Finished archive creation");
            }

            if config.post_hook.is_empty() {
                info!("Skipping post hook");
            } else {
                info!("Starting post hook");
                run_post_hook(&api, &config).await?;
                info!("Finished post hook");
            }
        }
    }

    Ok(())
}
