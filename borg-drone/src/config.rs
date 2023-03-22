//! The configuration definitions and parsing of lives here

use std::fs::{metadata, read_to_string};
use std::os::unix::fs::MetadataExt;

use log::warn;
use serde::{Deserialize, Serialize};
use url::Url;

/// The common settings for borg
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BorgConfig {
    /// The remote path of borg
    pub remote_path: Option<String>,
}

/// The configuration of borg-connect
///
/// The struct is deserialized from file
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// The path where the borg command can be found
    pub borg_path: String,
    /// The address of the [borg_vinculum] server.
    pub vinculum_address: Url,
    /// Borg specific configuration
    pub borg: BorgConfig,
}

/// Retrieve the config file from the given `config_path`.
///
/// If the file was not found or could not be deserialized, an error is returned.
pub fn get_config(config_path: &str) -> Result<Config, String> {
    let Ok(m) = metadata(config_path) else {
        return Err(format!("File {config_path} does not exist"));
    };

    if !m.is_file() {
        return Err(format!("{config_path} is not a file"));
    }

    if m.mode() != 0o600 || m.mode() != 0o400 {
        warn!("{config_path} has too broad permissions. 0600 or 0400 are recommended.");
    }

    let c = read_to_string(config_path).map_err(|e| format!("Couldn't read config file: {e}"))?;

    let config = toml::from_str(&c).map_err(|e| format!("Couldn't deserialize config: {e}"))?;

    Ok(config)
}
