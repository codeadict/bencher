use std::convert::TryFrom;

#[cfg(feature = "plus")]
use bencher_json::system::config::JsonPlus;
use bencher_json::{
    system::config::{
        IfExists, JsonConsole, JsonDatabase, JsonLogging, JsonSecurity, JsonServer, JsonSmtp,
        JsonTls, LogLevel, ServerLog,
    },
    JsonConfig,
};
use bencher_rbac::init_rbac;
use diesel::{connection::SimpleConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dropshot::{
    ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingIfExists, ConfigLoggingLevel,
    ConfigTls, HttpServer,
};
use slog::Logger;
use tokio::sync::mpsc::Sender;
use tracing::{trace, warn};

use crate::{
    context::{ApiContext, Database, DbConnection, Email, Messenger, SecretKey},
    endpoints::Api,
    error::api_error,
    ApiError,
};

#[cfg(feature = "plus")]
use super::plus::Plus;
use super::{Config, BENCHER_DOT_DEV, DEFAULT_SMTP_PORT};

const DATABASE_URL: &str = "DATABASE_URL";
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct ConfigTx {
    pub config: Config,
    pub restart_tx: Sender<()>,
}

impl TryFrom<ConfigTx> for HttpServer<ApiContext> {
    type Error = ApiError;

    fn try_from(config_tx: ConfigTx) -> Result<Self, Self::Error> {
        let ConfigTx { config, restart_tx } = config_tx;

        let Config(JsonConfig {
            endpoint,
            secret_key,
            console,
            security,
            server,
            database,
            smtp,
            logging,
            #[cfg(feature = "plus")]
            plus,
        }) = config;

        // TODO Remove deprecated endpoint
        let console = if let Some(console) = console {
            if endpoint.is_some() {
                warn!(
                    "DEPRECATED: The `endpoint` is now `console.url`. This value will be ignored."
                );
            }
            console
        } else if let Some(endpoint) = endpoint {
            warn!(
                "DEPRECATED: The `endpoint` is now `console.url`. This value will be used for now."
            );
            JsonConsole { url: endpoint }
        } else {
            return Err(ApiError::MissingConfigKey("console.url".into()));
        };

        // TODO Remove deprecated secret_key
        let security = if let Some(security) = security {
            if secret_key.is_some() {
                warn!(
                "DEPRECATED: The `secret_key` is now `security.secret_key`. This value will be ignored."
                );
            }
            security
        } else if let Some(secret_key) = secret_key {
            warn!(
                "DEPRECATED: The `secret_key` is now `security.secret_key`. This value will be used for now."
            );
            JsonSecurity {
                issuer: None,
                secret_key,
            }
        } else {
            return Err(ApiError::MissingConfigKey("security.secret_key".into()));
        };

        let private = into_private(
            console,
            security,
            smtp,
            database,
            restart_tx,
            #[cfg(feature = "plus")]
            plus,
        )?;
        let config_dropshot = into_config_dropshot(server);
        let log = into_log(logging)?;

        let mut api = ApiDescription::new();
        trace!("Registering server APIs");
        Api::register(&mut api, true)?;

        Ok(
            dropshot::HttpServerStarter::new(&config_dropshot, api, private, &log)
                .map_err(ApiError::CreateServer)?
                .start(),
        )
    }
}

fn into_private(
    console: JsonConsole,
    security: JsonSecurity,
    smtp: Option<JsonSmtp>,
    json_database: JsonDatabase,
    restart_tx: Sender<()>,
    #[cfg(feature = "plus")] plus: Option<JsonPlus>,
) -> Result<ApiContext, ApiError> {
    let endpoint = console.url.into();
    let database_path = json_database.file.to_string_lossy();
    diesel_database_url(&database_path);
    let mut database_connection = DbConnection::establish(&database_path)?;
    run_migrations(&mut database_connection)?;
    let data_store = if let Some(data_store) = json_database.data_store {
        Some(data_store.try_into()?)
    } else {
        None
    };
    let secret_key = SecretKey::new(
        security.issuer.unwrap_or_else(|| BENCHER_DOT_DEV.into()),
        security.secret_key,
    );
    #[cfg(feature = "plus")]
    let Plus { biller, licensor } = Plus::new(&endpoint, plus)?;

    Ok(ApiContext {
        endpoint,
        secret_key,
        rbac: init_rbac().map_err(ApiError::Polar)?.into(),
        messenger: into_messenger(smtp),
        database: Database {
            path: json_database.file,
            connection: tokio::sync::Mutex::new(database_connection),
            data_store,
        },
        restart_tx,
        #[cfg(feature = "plus")]
        biller,
        #[cfg(feature = "plus")]
        licensor,
    })
}

// Set the diesel `DATABASE_URL` env var to the database path
fn diesel_database_url(database_path: &str) {
    if let Ok(database_url) = std::env::var(DATABASE_URL) {
        if database_url == database_path {
            return;
        }
        trace!("\"{DATABASE_URL}\" ({database_url}) must be the same value as {database_path}");
    } else {
        trace!("Failed to find \"{DATABASE_URL}\"");
    }
    trace!("Setting \"{DATABASE_URL}\" to {database_path}");
    std::env::set_var(DATABASE_URL, database_path);
}

fn run_migrations(database: &mut DbConnection) -> Result<(), ApiError> {
    database
        .run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(ApiError::Migrations)?;
    // https://www.sqlite.org/foreignkeys.html#fk_enable
    database
        .batch_execute("PRAGMA foreign_keys = ON")
        .map_err(api_error!())?;
    Ok(())
}

fn into_messenger(smtp: Option<JsonSmtp>) -> Messenger {
    smtp.map_or(
        Messenger::StdOut,
        |JsonSmtp {
             hostname,
             port,
             starttls,
             username,
             secret,
             from_name,
             from_email,
         }| {
            Messenger::Email(Email {
                hostname: hostname.into(),
                port: port.unwrap_or(DEFAULT_SMTP_PORT),
                starttls: starttls.unwrap_or(true),
                username: username.into(),
                secret,
                from_name: Some(from_name.into()),
                from_email: from_email.into(),
            })
        },
    )
}

fn into_config_dropshot(server: JsonServer) -> ConfigDropshot {
    let JsonServer {
        bind_address,
        request_body_max_bytes,
        tls,
    } = server;
    ConfigDropshot {
        bind_address,
        request_body_max_bytes,
        tls: tls.map(|json_tls| match json_tls {
            JsonTls::AsFile {
                cert_file,
                key_file,
            } => ConfigTls::AsFile {
                cert_file,
                key_file,
            },
            JsonTls::AsBytes { certs, key } => ConfigTls::AsBytes { certs, key },
        }),
    }
}

fn into_log(logging: JsonLogging) -> Result<Logger, ApiError> {
    let JsonLogging { name, log } = logging;
    match log {
        ServerLog::StderrTerminal { level } => ConfigLogging::StderrTerminal {
            level: into_level(&level),
        },
        ServerLog::File {
            level,
            path,
            if_exists,
        } => ConfigLogging::File {
            level: into_level(&level),
            path: path.into(),
            if_exists: into_if_exists(&if_exists),
        },
    }
    .to_logger(name)
    .map_err(ApiError::CreateLogger)
}

fn into_level(log_level: &LogLevel) -> ConfigLoggingLevel {
    match log_level {
        LogLevel::Trace => ConfigLoggingLevel::Trace,
        LogLevel::Debug => ConfigLoggingLevel::Debug,
        LogLevel::Info => ConfigLoggingLevel::Info,
        LogLevel::Warn => ConfigLoggingLevel::Warn,
        LogLevel::Error => ConfigLoggingLevel::Error,
        LogLevel::Critical => ConfigLoggingLevel::Critical,
    }
}

fn into_if_exists(if_exists: &IfExists) -> ConfigLoggingIfExists {
    match if_exists {
        IfExists::Fail => ConfigLoggingIfExists::Fail,
        IfExists::Truncate => ConfigLoggingIfExists::Truncate,
        IfExists::Append => ConfigLoggingIfExists::Append,
    }
}
