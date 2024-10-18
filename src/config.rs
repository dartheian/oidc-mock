use axum::http::Uri;
use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use serde_with::base64::{Base64, Standard};
use serde_with::formats::Padded;
use serde_with::serde_as;
use thiserror::Error;

#[serde_as]
#[derive(Deserialize)]
pub struct Configuration {
    pub expiration: u64,
    pub ip: [u8; 4],
    #[serde(with = "http_serde::uri")]
    pub issuer: Uri,
    pub port: u16,
    pub rng_seed: u64,
    #[serde_as(as = "Base64<Standard, Padded>")]
    pub secret: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error while parsing the configuration: {0}")]
    Parse(#[from] ConfigError),
}

impl TryFrom<Environment> for Configuration {
    type Error = Error;

    fn try_from(environment: Environment) -> Result<Self, Self::Error> {
        let config = Config::builder().add_source(environment).build()?;
        let config = config.try_deserialize()?;
        Ok(config)
    }
}
