//! The API to the vinculum

use common::Stats;
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

/// The api definition for requests to the vinculum
#[derive(Clone)]
pub struct Api {
    address: Url,
    header: HeaderMap,
}

impl Api {
    /// Create a new api instance
    pub fn new(address: Url, token: &str) -> Result<Self, String> {
        let mut header = HeaderMap::new();
        header.insert(
            "Authorization",
            HeaderValue::try_from(&format!("Bearer {token}"))
                .map_err(|e| format!("Error while constructing headers for api: {e}"))?,
        );

        Ok(Self { address, header })
    }

    /// Send an error to the vinculum
    pub async fn send_error(&self, err: &str) -> Result<(), String> {
        Ok(())
    }

    /// Send stats to the vinculum
    pub async fn send_stats(&self, stats: Stats) -> Result<(), String> {
        Ok(())
    }
}
