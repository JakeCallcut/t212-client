// Storing and clearing Trading 212 API credentials on disk.
// Credentials are written to ~/.t212/creds.toml, this is a time trade-off

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{PathBuf};

#[derive(Serialize, Deserialize)]
struct StoredCreds {
    ///API Key and secret for t212 as stored in home dir
    key: String,
    secret: String,
}

/// Resolve ~/.t212/creds.toml.
fn creds_path() -> Result<PathBuf, AuthError> {
    let home = dirs::home_dir().ok_or(AuthError::NoHomeDir)?;
    Ok(home.join(".t212").join("creds.toml"))
}

/// auth command: prompt for key and secret, then store them.
pub fn run_auth() -> Result<(), AuthError> {
    let path = creds_path()?;

    if path.exists() {
        print!(
            "{} {}. Overwrite? [{}/{}] ",
            "Credentials already exist at".yellow(),
            path.display().bold(),
            "y".red(),
            "N".green().bold(),
        );
        io::stdout().flush()?;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        if !answer.trim().eq_ignore_ascii_case("y") {
            println!("Aborted. No changes made.");
            return Ok(());
        }
    }

    print!("{}","API key: ".blue().bold());
    io::stdout().flush()?;
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key = key.trim().to_string();

    // rpassword reads without echoing to the terminal.
    let secret = rpassword::prompt_password("API secret: ".blue().bold())?;
    let secret = secret.trim().to_string();

    if key.is_empty() || secret.is_empty() {
        return Err(AuthError::EmptyInput);
    }

    let contents = toml::to_string(&StoredCreds { key, secret })?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, &contents)?;

    println!("{} {}", "Credentials saved to ".green().bold(), path.display().bold());
    Ok(())
}

/// deauth command: delete the stored credentials.
pub fn run_deauth() -> Result<(), AuthError> {
    let path = creds_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
        println!("Credentials removed from {}", path.display());
    } else {
        println!("No stored credentials found. Nothing to remove.");
    }
    Ok(())
}

/// read stored credentials, returns None on any problem
pub fn load_from_file() -> Option<(String, String)> {
    let path = creds_path().ok()?;
    let contents = fs::read_to_string(path).ok()?;
    let stored: StoredCreds = toml::from_str(&contents).ok()?;
    Some((stored.key, stored.secret))
}

// errors

#[derive(Debug)]
pub enum AuthError {
    NoHomeDir,
    EmptyInput,
    Io(io::Error),
    Toml(toml::ser::Error),
}

impl From<io::Error> for AuthError {
    fn from(e: io::Error) -> Self {
        AuthError::Io(e)
    }
}
impl From<toml::ser::Error> for AuthError {
    fn from(e: toml::ser::Error) -> Self {
        AuthError::Toml(e)
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::NoHomeDir => write!(f, "could not determine your home directory"),
            AuthError::EmptyInput => write!(f, "key and secret must not be empty"),
            AuthError::Io(e) => write!(f, "file error: {e}"),
            AuthError::Toml(e) => write!(f, "could not serialise credentials: {e}"),
        }
    }
}

impl std::error::Error for AuthError {}