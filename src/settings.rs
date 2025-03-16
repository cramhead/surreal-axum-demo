
use std::fmt;

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use tracing::{Level, debug, instrument, span, trace, warn};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub db_name: String,
    pub timeout: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db_config: DbConfig,
    pub run_mode: ENV,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub enum ENV {
    #[default]
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl Settings {
    #[instrument(name = "load_settings", level = "debug", skip_all)]
    pub fn new() -> Result<Self, ConfigError> {
        let span = span!(Level::DEBUG, "building_config");
        let _enter = span.enter();

        debug!("Starting to build configuration.");

        let config_builder = Config::builder().add_source(
            File::with_name(".env")
                .required(false)
                .format(FileFormat::Ini),
        );

        debug!("Added .env file as a configuration source.");

        let config_builder = config_builder.add_source(Environment::with_prefix("APP"));

        debug!("Added environment variables with 'APP_' prefix as a configuration source.");

        let config_result = config_builder.build();
        trace!("Config builder created a config: {:?}", config_result); // This will only be logged if trace is enabled.
        let config = config_result.unwrap(); // As before, handle this error more gracefully in production code.

        let db_config_result = config.clone().try_deserialize::<DbConfig>();
        trace!("Config parsed into db_config: {:?}", db_config_result);
        let db_config = db_config_result.unwrap_or_default();

        let run_mode_result = config.get_string("run_mode");

        match &run_mode_result {
            Ok(run_mode) => {
                trace!("Run mode from config: {}", run_mode);
            }
            Err(e) => {
                warn!("Failed to get run_mode from config: {}", e);
            }
        }

        let run_mode = match run_mode_result.unwrap().as_str() {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        };
        Ok(Self {
            db_config,
            run_mode,
        })
    }
}
