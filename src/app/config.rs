//! Configuration file and environment parsing.
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};

use crate::app::cmdline::Args;

/// Root of configuration tree
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub http_server: HttpServer,
    pub server_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpServer {
    pub host: IpAddr,
    pub port: u16,
}

impl HttpServer {
    pub fn to_server_address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

impl Config {
    /// Load configuration from various sources and produce final configuration with following
    /// order of precedence (strong to weak).
    ///
    ///     1. environment variable
    ///     2. configuration file (if specified)
    ///     3. default configuration file (default.toml)
    pub fn load(args: &Args) -> crate::Result<Self> {
        // Step 1. load default configuration
        let mut figment = Config::default_figment();

        // Step 2, merge with specified config file
        if let Some(c) = args.config.as_deref() {
            figment = figment.merge(Toml::file(c));
        }

        // Step 3, merge with environment variable
        figment = figment.merge(env_provider());

        let config: Config = figment.extract()?;

        Ok(config)
    }

    pub fn to_toml_pretty(&self) -> Result<String, toml::ser::Error> {
        // N.B. We can't use toml::to_string_pretty(self) directly
        // Because toml will raise `ValueAfterTable` on some struct
        let value = toml::Value::try_from(self)?;

        toml::to_string_pretty(&value)
    }

    pub fn default_figment() -> Figment {
        Figment::from(Serialized::defaults(Self::default_value()))
    }

    pub fn default_value() -> Self {
        Config {
            http_server: HttpServer {
                // Default to match all ip
                host: IpAddr::from(Ipv4Addr::UNSPECIFIED),
                port: 8000,
            },
            server_name: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::default_value()
    }
}

fn env_provider() -> Env {
    Env::prefixed("REPLY__").split("__")
}
