//! The configuration of `borg-vinculum`

use std::fs::{read_to_string, File};
use std::net::IpAddr;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process::Command;

use actix_toolbox::logging::LoggingConfig;
use log::info;
use serde::{Deserialize, Serialize};

/// The configuration of all borg related settings
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BorgConfig {
    /// The path to the borg utility
    pub borg_path: String,
    /// The path to ssh key
    pub ssh_key_path: String,
    /// The path where the remote borg is found
    pub borg_remote_path: Option<String>,
}

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
    /// The borg related configuration
    pub borg: BorgConfig,
}

impl TryFrom<&Path> for Config {
    type Error = String;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let config_str =
            read_to_string(path).map_err(|e| format!("Could not read config file: {e}"))?;
        let conf: Config = toml::from_str(&config_str)
            .map_err(|e| format!("Error deserializing config from: {e}"))?;

        conf.validate()?;

        Ok(conf)
    }
}

impl Config {
    /// Validate the config
    fn validate(&self) -> Result<(), String> {
        if !Path::new(&self.borg.ssh_key_path).exists() {
            info!("Did not found ssh key, try to generate");
            let args = shlex::split(&format!(
                "-t ed25519 -f {path} -N ''",
                path = shlex::quote(&self.borg.ssh_key_path)
            ))
            .ok_or("Shlex error".to_string())?;
            let status = Command::new("ssh-keygen")
                .args(args)
                .status()
                .map_err(|e| format!("Error while executing ssh-keygen: {e}"))?;

            if !status.success() {
                return Err(format!(
                    "ssh-keygen returned with status code != 0: {}",
                    status.code().unwrap()
                ));
            }
        }

        let mode = File::open(&self.borg.ssh_key_path)
            .map_err(|e| format!("Could not open {p}: {e}", p = self.borg.ssh_key_path))?
            .metadata()
            .map_err(|e| format!("{e}"))?
            .mode();

        if mode & 0o177 != 0 {
            return Err(format!(
                "Mode of {p} is not 0600",
                p = self.borg.ssh_key_path
            ));
        }

        Ok(())
    }
}
