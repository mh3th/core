use anyhow::{Context, Result};
use dotenvy;
use std::env;

pub mod configuration;
pub use configuration::Configuration;

impl Configuration {
    pub fn from_env() -> Result<Self> {
        dotenvy::from_path_override(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env"))
            .context("Failed to load .env file")?;

        let main_port: u16 = env::var("MAIN_PORT")
            .context("Must be set to the MAIN_PORT server will listen on")?
            .parse()
            .context("The MAIN_PORT server port must be a valid u16")?;
        let metrics_port: u16 = env::var("METRICS_PORT")
            .context("Must be set to the METRICS_PORT server will listen on")?
            .parse()
            .context("The METRICS_PORT server port must be a valid u16")?;

        let database_url: String = env::var("DATABASE_URL")
            .context("Must be set to the DATABASE_URL server will connect to")?;

        Ok(Configuration {
            main_port,
            metrics_port,
            database_url,
        })
    }
}
