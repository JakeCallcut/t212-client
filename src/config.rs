// Contains all environment configuration including account type and credentials
#[derive(Default)]
pub enum Environment {
    /// the environment can either be a demo or live account
    #[default]
    Demo,
    Live,
}

//
impl Environment {
    /// based on environment enum, select the correct API endpoint
    pub fn base_url(self) -> &'static str {
        match self {
            Environment::Demo => "https://demo.trading212.com/api/v0",
            Environment::Live => "https://live.trading212.com/api/v0",
        }
    }
}

pub struct Credentials {
    ///holds the users Trading212 credentials
    pub key: String,
    pub secret: Option<String>,
}

impl Credentials {
    /// Read credentials from T212_API_KEY and T212_API_SECRET
    pub fn from_env() -> Result<Self, ConfigError> {
        let key = env::var("T212_API_KEY").map_err(|_| ConfigError::MissingApiKey)?;
        let secret = env::var("T212_API_SECRET").ok();
        Ok(Credentials { key, secret })
    }
}

pub struct Config {
    /// bundle credentials and environment into single class
    pub environment: Environment,
    pub credentials: Credentials,
}

impl Config {
    /// Build config for the given environment and creds
    pub fn from_env(environment: Environment) -> Result<Self, ConfigError> {
        Ok(Config {
            environment,
            credentials: Credentials::from_env()?,
        })
    }

    pub fn base_url(&self) -> &'static str {
        self.environment.base_url()
    }
}

pub enum ConfigError {
    MissingApiKey,
}