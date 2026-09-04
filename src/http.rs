// HTTP transport for the Trading 212 API.
// Read only, this only ever issues GET requests.

use crate::config::Config;
use base64::Engine;
use std::fmt;

pub struct Client {
    config: Config,
}

impl Client {
    //build client using current config, header containing keys
    pub fn new(config: Config) -> Self {
        Client { config }
    }

    fn auth_header(&self) -> String {
        let raw = format!(
            "{}:{}",
            self.config.credentials.key, self.config.credentials.secret
        );
        let encoded = base64::engine::general_purpose::STANDARD.encode(raw);
        format!("Basic {encoded}")
    }

    /// GET a path and return the raw body.
    pub fn get(&self, path: &str) -> Result<String, HttpError> {
        let url = format!("{}{}", self.config.base_url(), path);
        let auth = self.auth_header();

        match ureq::get(&url).header("Authorization", &auth).call() {
            Ok(mut response) => response
                .body_mut()
                .read_to_string()
                .map_err(|e| HttpError::Transport(e.to_string())),
            Err(ureq::Error::StatusCode(code)) => Err(HttpError::from_status(code)),
            Err(e) => Err(HttpError::Transport(e.to_string())),
        }
    }

    /// Verify credentials with one lightweight authenticated request.
    pub fn check_connection(&self) -> Result<(), HttpError> {
        self.get("/equity/account/summary")?;
        Ok(())
    }
}

//error definitions

#[derive(Debug)]
pub enum HttpError {
    Unauthorized,
    Forbidden,
    NotFound,
    RateLimited,
    Timeout,
    Server(u16),
    Unexpected(u16),
    Transport(String),
}

impl HttpError {
    fn from_status(code: u16) -> Self {
        match code {
            401 => HttpError::Unauthorized,
            403 => HttpError::Forbidden,
            404 => HttpError::NotFound,
            408 => HttpError::Timeout,
            429 => HttpError::RateLimited,
            500..=599 => HttpError::Server(code),
            other => HttpError::Unexpected(other),
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Unauthorized => write!(
                f,
                "authentication failed (401): Please check that you're on the right environment. use deauth and auth to re-establish keys"
            ),
            HttpError::Forbidden => write!(f, "forbidden (403): this key may lack the required scope"),
            HttpError::NotFound => write!(f, "not found (404): the endpoint may not exist on this environment"),
            HttpError::RateLimited => write!(f, "rate limited (429): too many requests, wait and retry"),
            HttpError::Timeout => write!(f, "request timed out (408)"),
            HttpError::Server(c) => write!(f, "server error ({c}): Trading 212 had a problem"),
            HttpError::Unexpected(c) => write!(f, "unexpected status code: {c}"),
            HttpError::Transport(m) => write!(f, "network error: {m}"),
        }
    }
}

impl std::error::Error for HttpError {}