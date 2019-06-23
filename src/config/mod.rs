use log::debug;
use serde::{Deserialize, Serialize};

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

impl Drop for Config {
    fn drop(&mut self) {
        println!("Config.drop");
    }
}

pub fn read(path: &str) -> Result<Config, Box<std::error::Error>> {
    let reader = std::fs::File::open(path)?;

    let config: Config = serde_yaml::from_reader(reader)?;
    debug!("config={:#?}", config);

    Ok(config)
}
