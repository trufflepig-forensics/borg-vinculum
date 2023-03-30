//! The configuration of `borg-vinculum`

use std::fs::read_to_string;
use std::net::IpAddr;

use actix_toolbox::logging::LoggingConfig;
use serde::{Deserialize, Serialize};

/// The configuration of the connection to a matrix server
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MatrixConfig {
    /// The url to a homeserver
    pub homeserver: String,
    /// The username that should be used for login
    pub username: String,
    /// The password that should be used for login
    pub password: String,
    /// Channel to send info messages to
    pub channel: String,
}

/// Configuration regarding the server
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ServerConfig {
    /// The address the server should bind to
    pub listen_address: IpAddr,
    /// The port the server should bind to
    pub listen_port: u16,
    /// Base64 encoded secret key
    ///
    /// The key is used to sign and verify sessions.
    ///
    /// Do not expose this key!
    pub secret_key: String,
}

/// Configuration regarding the database
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DBConfig {
    /// Host the database is located on
    pub host: String,
    /// Port the database is located on
    pub port: u16,
    /// The name of the database to connect to.
    pub name: String,
    /// The username to use for the database connection
    pub user: String,
    /// The password to use for the database connection
    pub password: String,
}

/// The configuration file of borg-vinculum
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// Configuration regarding the server
    pub server: ServerConfig,
    /// The logging configuration
    pub logging: LoggingConfig,
    /// The database configuration
    pub database: DBConfig,
    /// The matrix configuration
    pub matrix: MatrixConfig,
}

impl Config {
    /// Retrieve a config file using the `path` variable.
    pub fn from_path(path: &str) -> Result<Config, String> {
        let config_str = read_to_string(path)
            .map_err(|e| format!("Could not read config file from {path}: {e}"))?;
        toml::from_str(&config_str).map_err(|e| format!("Error deserializing config from: {e}"))
    }
}
