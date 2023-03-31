//! # borg-vinculum
//!
//! The control unit of all borg-drones.
#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use actix_toolbox::logging::setup_logging;
use actix_web::cookie::Key;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use base64::prelude::{Engine, BASE64_STANDARD};
use clap::{Parser, Subcommand};
use rand::thread_rng;
use rorm::{cli, insert, query, Database, DatabaseConfiguration, DatabaseDriver, Model};
use uuid::Uuid;

use crate::chan::start_matrix_notifier;
use crate::config::Config;
use crate::models::{Account, AccountInsert};
use crate::modules::matrix::MatrixApi;

pub(crate) mod chan;
pub mod config;
pub mod handler;
pub(crate) mod middleware;
pub mod models;
pub mod modules;
pub mod server;
pub(crate) mod swagger;

/// The subcommands of the vinculum
#[derive(Subcommand)]
pub enum Command {
    /// Start the vinculum
    Start,
    /// Generate a secret key
    Keygen,
    /// Apply migrations
    Migrate {
        /// The path to the migration directory
        #[clap(default_value_t = String::from("./migrations/"))]
        migration_dir: String,
    },
    /// Test the connection to the matrix server
    TestMatrix,
    /// Create an account via cli
    CreateAccount,
}

/// The control unit of all borg-drones
#[derive(Parser)]
pub struct Cli {
    /// The path to the configuration file of the vinculum
    #[clap(long, default_value_t = String::from("/etc/vinculum/config.toml"))]
    config_path: String,

    #[clap(subcommand)]
    command: Command,
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start => {
            let conf = Config::try_from(Path::new(&cli.config_path))?;
            setup_logging(&conf.logging)?;

            let db = get_db(&conf).await?;

            let matrix = MatrixApi::new(conf.matrix.homeserver.clone().parse().unwrap());
            let matrix_notifier_chan = start_matrix_notifier(&conf, matrix).await?;

            server::start_server(&conf, db, matrix_notifier_chan).await?;
        }
        Command::Keygen => {
            let key = Key::generate();
            println!("{}", BASE64_STANDARD.encode(key.master()));
        }
        Command::Migrate { migration_dir } => {
            let conf = Config::try_from(Path::new(&cli.config_path))?;
            setup_logging(&conf.logging)?;

            cli::migrate::run_migrate_custom(
                cli::config::DatabaseConfig {
                    last_migration_table_name: None,
                    driver: DatabaseDriver::Postgres {
                        host: conf.database.host,
                        port: conf.database.port,
                        name: conf.database.name,
                        user: conf.database.user,
                        password: conf.database.password,
                    },
                },
                migration_dir,
                false,
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
        }
        Command::TestMatrix => {
            let conf = Config::try_from(Path::new(&cli.config_path))?;
            setup_logging(&conf.logging)?;

            let mut matrix = MatrixApi::new(conf.matrix.homeserver.parse().unwrap());
            matrix
                .login(&conf.matrix.username, &conf.matrix.password)
                .await
                .map_err(|e| format!("Error login into matrix account: {e}"))?;

            matrix
                .join_room(&conf.matrix.channel)
                .await
                .map_err(|e| format!("Error joining into matrix room: {e}"))?;

            matrix
                .send_message(
                    "The Vinculum announces: Alarm!\n\nThis is a test!".to_string(),
                    Some(
                        r#"<h3>🚨 🚨 🚨 The Vinculum announces Alarm! 🚨 🚨 🚨</h3><p>This is a test!</p>"#
                        .to_string(),
                    ),
                    &conf.matrix.channel,
                )
                .await
                .map_err(|e| format!("Error sending message to configured channel: {e}"))?;
        }
        Command::CreateAccount => {
            let conf = Config::try_from(Path::new(&cli.config_path))?;
            setup_logging(&conf.logging)?;
            let db = get_db(&conf).await?;

            let stdin = io::stdin();
            let mut stdout = io::stdout();

            let mut username = String::new();

            print!("Enter a username: ");
            stdout.flush().unwrap();
            stdin.read_line(&mut username).unwrap();
            let username = username.trim();

            if username.is_empty() {
                eprintln!("Empty username is not allowed");
                exit(1);
            }

            if query!(&db, (Account::F.username,))
                .condition(Account::F.username.equals(username))
                .optional()
                .await
                .unwrap()
                .is_some()
            {
                eprintln!("There is already an account with that name");
                exit(1);
            }

            let password = rpassword::prompt_password("Enter password: ").unwrap();
            if password.is_empty() {
                eprintln!("Empty password is not allowed");
                exit(1);
            }

            let salt = SaltString::generate(&mut thread_rng());
            let hashed_password = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            insert!(&db, AccountInsert)
                .single(&AccountInsert {
                    username: username.to_string(),
                    password_hash: hashed_password,
                    uuid: Uuid::new_v4(),
                })
                .await
                .map_err(|e| format!("Failed to create account: {e}"))?;

            println!("Created account {username}");
        }
    }

    Ok(())
}

/// Retrieves the database using the provided config.
///
/// If the connection fails, an error is returned
async fn get_db(config: &Config) -> Result<Database, String> {
    let c = DatabaseConfiguration {
        driver: DatabaseDriver::Postgres {
            host: config.database.host.clone(),
            port: config.database.port,
            name: config.database.name.clone(),
            user: config.database.user.clone(),
            password: config.database.password.clone(),
        },
        min_connections: 2,
        max_connections: 20,
        disable_logging: Some(true),
        statement_log_level: None,
        slow_statement_log_level: None,
    };

    Database::connect(c)
        .await
        .map_err(|e| format!("Error connecting to database: {e}"))
}
