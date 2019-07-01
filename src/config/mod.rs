use log::debug;
use serde::{Deserialize, Serialize};

// https://github.com/dtolnay/serde-yaml/issues/94
// https://github.com/dtolnay/serde-yaml/issues/93
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub openid_configuration: String,
    pub financial_id: String,
    pub ssid: String,
    pub organisation_id: String,
    pub ssa: String,
    pub kid: String,
    pub client_id: String,
    pub request_object_signing_alg: String,
    pub encryption_private: String,
    pub encryption_public: String,
    pub signature_private: String,
    pub signature_public: String,
    pub transport_private: String,
    pub transport_public: String,
    pub register_response: String,
}

impl std::str::FromStr for Config {
    type Err = Box<std::error::Error>;

    /// Load a `Config` from some string.
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let config: Config = serde_yaml::from_str(src)?;
        Ok(config)
    }
}

pub fn read<S: Into<String>>(path: S) -> Result<Config, Box<std::error::Error>> {
    let reader = std::fs::File::open(path.into().as_str())?;

    let config: Config = serde_yaml::from_reader(reader)?;
    debug!("config={:#?}", config);

    Ok(config)
}
