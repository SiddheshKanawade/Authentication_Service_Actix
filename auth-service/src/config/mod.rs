use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32
}

impl Config {

    // this will fetch data from the .env folder and return a "Config" datatype defined above
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        // Logger
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading Configuration");

        //config crate helps to bring configurations from different sources and merge into one "Config"
        let mut c = config::Config::new();

        c.merge(config::Environment::default())?; // "?" helps in formating the errors

        // converts the file into our defined struct
        c.try_into()  
            .context("Loading configuration from .env")
    }
}