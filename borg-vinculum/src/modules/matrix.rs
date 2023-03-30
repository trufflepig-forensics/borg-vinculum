//! The matrix client API is here

use std::fmt::{Display, Formatter};
use std::time::Duration;

use log::warn;
use rand::distributions::{Alphanumeric, DistString};
use reqwest::header::HeaderMap;
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};
use url::ParseError;

#[derive(Serialize, Deserialize)]
struct LoginIdentifier {
    #[serde(rename = "type")]
    pub message_type: String,
    pub user: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    #[serde(rename = "type")]
    pub message_type: String,
    pub identifier: LoginIdentifier,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    pub user_id: String,
    pub access_token: String,
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
struct SendMessageRequest {
    body: String,
    msgtype: String,
    format: Option<String>,
    formatted_body: Option<String>,
}

/// The API for the client-server API
#[derive(Clone)]
pub struct MatrixApi {
    access_token: Option<String>,
    homeserver: Url,
    client: reqwest::Client,
}

impl MatrixApi {
    /// Create a new API client
    pub fn new(homeserver: Url) -> Self {
        Self {
            access_token: None,
            homeserver,
            client: Default::default(),
        }
    }

    async fn check_res(&self, res: Response) -> Result<Response, MatrixError> {
        if res.status() == 400 {
            warn!("Received bad request: {text}", text = res.text().await?);
            return Err(MatrixError::BadRequest);
        } else if res.status() == 403 {
            return Err(MatrixError::LoginFailed);
        } else if res.status() == 429 {
            return Err(MatrixError::RateLimited);
        } else if res.status() != 200 {
            warn!(
                "Received status code: {code}: {text}",
                code = res.status(),
                text = res.text().await?
            );
            return Err(MatrixError::Unknown);
        }

        Ok(res)
    }

    fn get_header(&self) -> Result<HeaderMap, MatrixError> {
        let mut hm = HeaderMap::new();
        if let Some(access_token) = &self.access_token {
            hm.insert(
                "Authorization",
                format!("Bearer {access_token}").parse().unwrap(),
            );
        }

        Ok(hm)
    }

    /// Perform a login
    pub async fn login(&mut self, username: String, password: String) -> Result<(), MatrixError> {
        let lr = LoginRequest {
            message_type: "m.login.password".to_string(),
            identifier: LoginIdentifier {
                user: username,
                message_type: "m.id.user".to_string(),
            },
            password,
        };
        let res = self
            .client
            // Safe as manually checked
            .post(self.homeserver.join("/_matrix/client/v3/login").unwrap())
            .json(&lr)
            .timeout(Duration::from_secs(3))
            .send()
            .await?;

        let res = self.check_res(res).await?;

        let res: LoginResponse = res.json().await?;
        self.access_token = Some(res.access_token);

        Ok(())
    }

    /// Join a room with a given ID
    pub async fn join_room(&self, room_id: &str) -> Result<(), MatrixError> {
        let res = self
            .client
            .post(
                self.homeserver
                    .join(&format!("/_matrix/client/v3/join/{room_id}"))?,
            )
            .headers(self.get_header()?)
            .send()
            .await?;

        self.check_res(res).await?;

        Ok(())
    }

    /// Send a message to the configured channel
    pub async fn send_message(
        &self,
        message: String,
        formatted_message: Option<String>,
        room_id: &str,
    ) -> Result<(), MatrixError> {
        let tx = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let msg = SendMessageRequest {
            body: message,
            msgtype: "m.text".to_string(),
            format: if formatted_message.is_some() {
                Some("org.matrix.custom.html".to_string())
            } else {
                None
            },
            formatted_body: formatted_message,
        };

        let res = self
            .client
            .put(self.homeserver.join(&format!(
                "/_matrix/client/v3/rooms/{room_id}/send/m.room.message/{tx}"
            ))?)
            .json(&msg)
            .headers(self.get_header()?)
            .send()
            .await?;

        self.check_res(res).await?;

        Ok(())
    }
}

/// The errors that can be thrown by a [MatrixApi]
#[derive(Debug)]
pub enum MatrixError {
    /// The login failed
    LoginFailed,
    /// Received a bad request
    BadRequest,
    /// The request was rate limited
    RateLimited,
    /// An unknown error occurred
    Unknown,
    /// Reqwest internal error
    Reqwest(reqwest::Error),
    /// Error while parsing url
    InvalidUrl(ParseError),
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::LoginFailed => write!(f, "Login failed"),
            MatrixError::BadRequest => write!(f, "Bad request"),
            MatrixError::RateLimited => write!(f, "Run into rate limit"),
            MatrixError::Unknown => write!(f, "unknown error"),
            MatrixError::Reqwest(err) => write!(f, "Reqwest error: {err}"),
            MatrixError::InvalidUrl(err) => write!(f, "Error parsing url: {err}"),
        }
    }
}

impl From<ParseError> for MatrixError {
    fn from(value: ParseError) -> Self {
        Self::InvalidUrl(value)
    }
}

impl From<reqwest::Error> for MatrixError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
