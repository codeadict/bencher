use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bencher_json::{
    sanitize_json,
    system::config::{
        JsonConsole, JsonDatabase, JsonLogging, JsonSecurity, JsonServer, LogLevel, ServerLog,
    },
    JsonConfig, Secret,
};
use once_cell::sync::Lazy;
use tracing::{error, info};
use url::Url;

use crate::ApiError;

pub mod config_tx;
#[cfg(feature = "plus")]
mod plus;

pub const API_NAME: &str = "Bencher API";

pub const BENCHER_CONFIG: &str = "BENCHER_CONFIG";
pub const BENCHER_CONFIG_PATH: &str = "BENCHER_CONFIG_PATH";

const DEFAULT_CONFIG_PATH: &str = "bencher.json";
const DEFAULT_CONSOLE_URL_STR: &str = "http://localhost:3000";
// Dynamic and/or Private Ports (49152-65535)
// https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml?search=61016
const DEFAULT_PORT: u16 = 61016;
// 1 megabyte or 1_048_576 bytes
const DEFAULT_MAX_BODY_SIZE: usize = 2 << 19;
const DEFAULT_DB_PATH: &str = "data/bencher.db";
const DEFAULT_SMTP_PORT: u16 = 587;
const BENCHER_DOT_DEV: &str = "bencher.dev";

#[cfg(debug_assertions)]
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
#[cfg(not(debug_assertions))]
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

#[allow(clippy::panic)]
static DEFAULT_CONSOLE_URL: Lazy<Url> = Lazy::new(|| {
    DEFAULT_CONSOLE_URL_STR.parse().unwrap_or_else(|e| {
        panic!("Failed to parse default console URL \"{DEFAULT_CONSOLE_URL_STR}\": {e}")
    })
});

static DEFAULT_BIND_ADDRESS: Lazy<SocketAddr> =
    Lazy::new(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), DEFAULT_PORT));

#[cfg(debug_assertions)]
#[allow(clippy::expect_used)]
pub static DEFAULT_SECRET_KEY: Lazy<Secret> = Lazy::new(|| {
    "DO_NOT_USE_THIS_IN_PRODUCTION"
        .parse()
        .expect("Invalid secret key")
});
#[cfg(not(debug_assertions))]
pub static DEFAULT_SECRET_KEY: Lazy<Secret> = Lazy::new(|| uuid::Uuid::new_v4().into());

#[derive(Debug, Clone)]
pub struct Config(JsonConfig);

impl Config {
    pub async fn load_or_default() -> Result<Self, ApiError> {
        if let Some(config) = Self::load_env().await? {
            return Ok(config);
        }

        if let Some(config) = Self::load_file().await? {
            return Ok(config);
        }

        let config = Self::default();
        info!("Using default config: {}", sanitize_json(&config.0));
        let config_str = if cfg!(debug_assertions) {
            serde_json::to_string_pretty(&config.0)
        } else {
            serde_json::to_string(&config.0)
        }?;
        Self::write(config_str.as_bytes()).await?;

        Ok(config)
    }

    pub async fn load_env() -> Result<Option<Self>, ApiError> {
        // If the env var is set then failing to read or parse the config is an error
        // However, if it isn't set then just return None
        let config_str = match std::env::var(BENCHER_CONFIG) {
            Ok(config_str) => config_str,
            Err(e) => {
                info!("Failed to find \"{BENCHER_CONFIG}\" environment variable: {e}");
                return Ok(None);
            },
        };

        let json_config = serde_json::from_str(&config_str).map_err(|e| {
            error!("Failed to parse config string from \"{BENCHER_CONFIG}\": {e}");
            ApiError::ParseConfigString(config_str.clone())
        })?;
        info!(
            "Loaded config from env var \"{BENCHER_CONFIG}\": {}",
            sanitize_json(&json_config)
        );

        #[cfg(debug_assertions)]
        Self::write(config_str.as_bytes()).await?;

        Ok(Some(Self(json_config)))
    }

    pub async fn load_file() -> Result<Option<Self>, ApiError> {
        // If the env var is set then failing to read or parse the config is an error
        // However, if it isn't set then just try the default path
        // If there is a file to read at the default path, then that config is expected to parse
        // Otherwise, just return None if there is no file to read a the default path
        let (path, config_file) = match std::env::var(BENCHER_CONFIG_PATH) {
            Ok(path) => {
                let config_file = tokio::fs::read(&path).await.map_err(|e| {
                    error!("Failed to open config file at {}: {e}", path);
                    ApiError::OpenConfigFile(path.clone())
                })?;
                (path, config_file)
            },
            Err(e) => {
                info!("Failed to find \"{BENCHER_CONFIG_PATH}\" environment variable defaulting to \"{DEFAULT_CONFIG_PATH}\": {e}");
                let config_file = match tokio::fs::read(DEFAULT_CONFIG_PATH).await {
                    Ok(config_file) => config_file,
                    Err(e) => {
                        info!("Failed to open config file at default path \"{DEFAULT_CONFIG_PATH}\": {e}");
                        return Ok(None);
                    },
                };
                (DEFAULT_CONFIG_PATH.into(), config_file)
            },
        };

        let json_config = serde_json::from_slice(&config_file).map_err(|e| {
            error!("Failed to parse config file at {path}: {e}");
            ApiError::ParseConfigFile(path.clone())
        })?;
        info!(
            "Loaded config from file {path}: {}",
            sanitize_json(&json_config)
        );

        Ok(Some(Self(json_config)))
    }

    pub async fn write(config: impl AsRef<[u8]>) -> Result<(), ApiError> {
        let path = std::env::var(BENCHER_CONFIG_PATH).unwrap_or_else(|e| {
            info!("Failed to find \"{BENCHER_CONFIG_PATH}\" environment variable defaulting to \"{DEFAULT_CONFIG_PATH}\": {e}");
            DEFAULT_CONFIG_PATH.into()
        });

        tokio::fs::write(&path, config).await.map_err(|e| {
            error!("Failed to write config file at {path}: {e}");
            ApiError::WriteConfigFile(path)
        })
    }

    pub fn into_inner(self) -> JsonConfig {
        self.0
    }
}

impl Default for Config {
    fn default() -> Self {
        Self(JsonConfig {
            endpoint: None,
            secret_key: None,
            console: Some(JsonConsole {
                url: DEFAULT_CONSOLE_URL.clone().into(),
            }),
            security: Some(JsonSecurity {
                issuer: Some(BENCHER_DOT_DEV.into()),
                secret_key: DEFAULT_SECRET_KEY.clone(),
            }),
            server: JsonServer {
                bind_address: *DEFAULT_BIND_ADDRESS,
                request_body_max_bytes: DEFAULT_MAX_BODY_SIZE,
                tls: None,
            },
            database: JsonDatabase {
                file: DEFAULT_DB_PATH.into(),
                data_store: None,
            },
            smtp: None,
            logging: JsonLogging {
                name: API_NAME.into(),
                log: ServerLog::StderrTerminal {
                    level: DEFAULT_LOG_LEVEL,
                },
            },
            #[cfg(feature = "plus")]
            plus: None,
        })
    }
}

impl From<Config> for JsonConfig {
    fn from(config: Config) -> Self {
        config.0
    }
}
