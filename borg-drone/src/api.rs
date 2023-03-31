//! The API to the vinculum

use std::time::Duration;

use common::{CreateStats, ErrorReport, HookStats, StatReport};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct ErrorMessage {
    message: String,
    code: u16,
}

/// The api definition for requests to the vinculum
#[derive(Clone)]
pub struct Api {
    address: Url,
    client: reqwest::Client,
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

        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .default_headers(header)
            .build()
            .map_err(|e| format!("Could not create API client: {e}"))?;

        Ok(Self { address, client })
    }

    async fn check_error(&self, res: Response) -> Result<(), String> {
        if res.status() != 200 {
            return if res.status() == 400 || res.status() == 500 {
                let error: ErrorMessage = res
                    .json()
                    .await
                    .map_err(|e| format!("Could not deserialize error response: {e}"))?;

                Err(format!(
                    "Error code {code}: {msg}",
                    code = error.code,
                    msg = error.message
                ))
            } else {
                let x = res
                    .text()
                    .await
                    .map_err(|e| format!("Could not convert response to text: {e}"))?;

                Err(format!("Unknown error returned: {x}"))
            };
        }

        Ok(())
    }

    /// Send an error to the vinculum
    pub async fn send_error(&self, error_report: ErrorReport) -> Result<(), String> {
        let res = self
            .client
            .post(self.address.join("/api/drone/v1/error").unwrap())
            .json(&error_report)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        self.check_error(res).await?;

        Ok(())
    }

    /// Send stats to the vinculum
    pub async fn send_stats(
        &self,
        pre_hook_stats: Option<HookStats>,
        create_stats: CreateStats,
        post_hook_stats: Option<HookStats>,
    ) -> Result<(), String> {
        let stat_report = StatReport {
            pre_hook_stats,
            create_stats,
            post_hook_stats,
        };

        let res = self
            .client
            .post(self.address.join("/api/drone/v1/stats").unwrap())
            .json(&stat_report)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        self.check_error(res).await?;

        Ok(())
    }
}
