//! Runtime configuration: environment selection and credential resolution.

use std::fmt;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    ///Environment can either be live or paper trading account type
    #[default]
    Demo,
    Live,
}

impl Environment {
    /// choose endpoint based on account type
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Demo => "https://demo.trading212.com/api/v0",
            Environment::Live => "https://live.trading212.com/api/v0",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Credentials {
    ///credentials structure contains both key and secret
    pub key: String,
    pub secret: String,
}

impl Credentials {
    /// get credentials from file written by `t212 auth`, otherwise error.
    pub fn resolve() -> Result<Self, ConfigError> {
        if let Some((key, secret)) = crate::auth::load_from_file() {
            return Ok(Credentials { key, secret });
        }
        Err(ConfigError::NoCredentials)
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    ///config struct to bundle environment and credentials into one
    pub environment: Environment,
    pub credentials: Credentials,
}

impl Config {
    //resolve credentials and environment into struct
    pub fn load(environment: Environment) -> Result<Self, ConfigError> {
        Ok(Config {
            environment,
            credentials: Credentials::resolve()?,
        })
    }

    pub fn base_url(&self) -> &'static str {
        self.environment.base_url()
    }
}

//errors
#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    NoCredentials,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NoCredentials => write!(
                f,
                "no credentials found: run `t212 auth`, or set T212_API_KEY and T212_API_SECRET"
            ),
        }
    }
}

impl std::error::Error for ConfigError {}