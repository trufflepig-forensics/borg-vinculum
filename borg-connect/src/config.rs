//! The configuration definitions and parsing of lives here

use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

/// The configuration of borg-connect
///
/// The struct is deserialized from file
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// The path where the borg command can be found
    pub borg_path: String,
}

/// Retrieve the config file from the given `config_path`.
///
/// If the file was not found or could not be deserialized, an error is returned.
pub fn get_config(config_path: &str) -> Result<Config, String> {
    let c = read_to_string(config_path).map_err(|e| format!("Couldn't read config file: {e}"))?;

    let config = toml::from_str(&c).map_err(|e| format!("Couldn't deserialize config: {e}"))?;

    Ok(config)
}
