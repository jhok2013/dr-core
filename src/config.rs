//! Configuration loading and handling with environment variables
use crate::error::Error;
use crate::secrets::{Mask, Secret};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, fmt::Display};
use url::Url;

/// Default environment variable names
pub enum Defaults {
    Config,
    Profile,
    Username,
    Host,
    Port,
    Pwd,
    Token,
}

impl Display for Defaults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let envar = match self {
            Defaults::Config => "DR_CONFIG",
            Defaults::Profile => "DR_PROFILE",
            Defaults::Username => "DR_USER",
            Defaults::Host => "DR_HOST",
            Defaults::Port => "DR_PORT",
            Defaults::Pwd => "DR_PWD",
            Defaults::Token => "DR_TOKEN",
        };
        write!(f, "{}", envar)
    }
}

impl From<Defaults> for String {
    fn from(value: Defaults) -> Self {
        format!("{}", value)
    }
}

impl From<&Defaults> for String {
    fn from(value: &Defaults) -> Self {
        format!("{}", value)
    }
}

/// Loads a config RON file at the designated path
pub fn load(path: &Path) -> Result<HashMap<String, Profile>, Error> {
    fs::read_to_string(path).map_err(Error::from).and_then(|s| {
        ron::from_str(&s)
            .map_err(|e| e.to_string())
            .map_err(Error::from)
    })
}

/// Loads the conf file at DR_CONFIG environment variable
pub fn default() -> Result<HashMap<String, Profile>, Error> {
    let def = env::var(Defaults::Config.to_string()).map_err(Error::from)?;
    let path = Path::new(&def);
    load(path)
}

/// Credentials profile for connecting to Dremio
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub username: String,
    pub secret: Secret<String>,
    pub host: Url,
}

impl Profile {
    /// Attempts to load the default profile from the default config RON file
    pub fn try_default() -> Result<Self, Error> {
        let dr_config = env::var(Defaults::Config.to_string())?;
        let path = Path::new(&dr_config);
        let name = env::var(Defaults::Profile.to_string())?;
        Profile::load_with(path, &name)
    }

    /// Loads a given profile from the designated file path
    pub fn load_with(path: &Path, name: &str) -> Result<Self, Error> {
        let conf = load(path)?;
        let profile = conf
            .get(name)
            .ok_or(format!("Profile {} is not in {:?}", name, path))?;
        Ok(profile.to_owned())
    }

    /// Creates a new Profile
    pub fn new(username: &str, secret: Secret<String>, host: Url) -> Self {
        Profile {
            username: String::from(username),
            secret,
            host,
        }
    }

    /// Creates a new Profile from the DR environment variables
    pub fn from_envars(
        username: Option<&str>,
        token: Option<Secret<String>>,
        pwd: Option<Secret<String>>,
        host: Option<Url>,
        port: Option<u16>,
    ) -> Result<Self, Error> {
        let username = match username {
            Some(val) => String::from(val),
            None => env::var(Defaults::Username.to_string())?,
        };
        let secret = match (token, pwd) {
            (Some(inner), None) => match inner {
                Secret::Token(mask) => Secret::Token(mask),
                Secret::Pwd(mask) => Secret::Token(mask),
            },
            (None, Some(inner)) => match inner {
                Secret::Token(mask) => Secret::Pwd(mask),
                Secret::Pwd(mask) => Secret::Pwd(mask),
            },
            (Some(t), Some(_)) => match t {
                Secret::Token(mask) => Secret::Token(mask),
                Secret::Pwd(mask) => Secret::Token(mask),
            },
            (None, None) => match env::var(Defaults::Token.to_string()) {
                Ok(token_val) => Secret::Token(Mask::from(token_val)),
                Err(_) => Secret::Pwd(Mask::from(env::var(Defaults::Pwd.to_string())?)),
            },
        };
        let mut host = match host {
            Some(val) => val,
            None => Url::parse(&env::var(Defaults::Host.to_string())?)?,
        };
        let port = match port {
            Some(p) => Some(p),
            None => env::var(Defaults::Port.to_string())
                .ok()
                .and_then(|s| s.parse::<u16>().ok()),
        };
        if let Some(p) = port {
            host.set_port(Some(p)).expect("Should have set the port");
        }
        Ok(Profile::new(&username, secret, host))
    }
}
