//! The configuration of `borg-vinculum`

use std::fs::{read_to_string, File};
use std::net::IpAddr;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use actix_toolbox::logging::LoggingConfig;
use log::info;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use ssh_key::{Algorithm, LineEnding, PrivateKey};

/// The configuration of all borg related settings
#[derive(Deserialize, Serialize, Clone, Debug)]
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
#[derive(Deserialize, Serialize, Clone, Debug)]
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
#[derive(Deserialize, Serialize, Clone, Debug)]
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
#[derive(Deserialize, Serialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    /// The private key
    #[serde(skip)]
    pub private_key: Option<PrivateKey>,
}

impl TryFrom<&Path> for Config {
    type Error = String;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let config_str =
            read_to_string(path).map_err(|e| format!("Could not read config file: {e}"))?;
        let mut conf: Config = toml::from_str(&config_str)
            .map_err(|e| format!("Error deserializing config from: {e}"))?;

        let pk = retrieve_ssh_key(&conf)?;
        conf.private_key = Some(pk);

        Ok(conf)
    }
}

/// Retrieve a ssh private
///
/// It uses the provided [Config] to retrieve a ssh private key.
/// If it doesn't yet exist, it is generated
fn retrieve_ssh_key(conf: &Config) -> Result<PrivateKey, String> {
    let pk_path = Path::new(&conf.borg.ssh_key_path);

    let private_key = if !pk_path.exists() {
        info!("Did not found ssh key, try to generate");
        let private_key = PrivateKey::random(&mut thread_rng(), Algorithm::Ed25519)
            .map_err(|e| format!("Error generating ssh key: {e}"))?;

        private_key
            .write_openssh_file(pk_path, LineEnding::LF)
            .map_err(|e| format!("Error writing ssh key: {e}"))?;

        private_key
    } else {
        PrivateKey::read_openssh_file(pk_path).map_err(|e| format!("Error reading ssh key: {e}"))?
    };

    let mode = File::open(pk_path)
        .map_err(|e| format!("Could not open {p}: {e}", p = conf.borg.ssh_key_path))?
        .metadata()
        .map_err(|e| format!("{e}"))?
        .mode();

    if mode & 0o177 != 0 {
        return Err(format!(
            "Mode of {p} is not 0600",
            p = conf.borg.ssh_key_path
        ));
    }

    Ok(private_key)
}
