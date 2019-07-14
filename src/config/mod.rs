use log::debug;
use serde;

// https://github.com/dtolnay/serde-yaml/issues/94
// https://github.com/dtolnay/serde-yaml/issues/93
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Default,
    Clone,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub openid_configuration:       String,
    pub financial_id:               String,
    pub ssid:                       String,
    pub organisation_id:            String,
    pub ssa:                        String,
    pub kid:                        String,
    pub client_id:                  String,
    pub request_object_signing_alg: String,
    pub encryption_private:         String,
    pub encryption_public:          String,
    pub signature_private:          String,
    pub signature_public:           String,
    pub transport_private:          String,
    pub transport_public:           String,
    pub register_response:          String,
}

impl Config {
    pub fn read<S: Into<String>>(path: S) -> Result<Self, Box<std::error::Error>>
    where
        S: std::fmt::Debug,
    {
        use std::convert::TryFrom;

        debug!("path={:#?}", path);
        let src = std::fs::read_to_string(path.into())?;
        let config = Config::try_from(src.as_str())?;

        Ok(config)
    }
}

impl std::convert::TryFrom<&str> for Config {
    type Error = Box<std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let config = serde_yaml::from_str(value)?;
        Ok(config)
    }
}

impl std::str::FromStr for Config {
    type Err = Box<std::error::Error>;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let config = serde_yaml::from_str(src)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    // Use: https://doc.rust-lang.org/stable/std/panic/fn.catch_unwind.html

    #[test]
    fn test_read_config_good() {
        let config_good = super::Config::read("src/config/testdata/config_good.yml")
            .expect("src/config/testdata/config_good.yml");
        println!("config_good={:?}", config_good);
    }

    #[test]
    #[should_panic]
    fn test_read_config_bad() {
        let config_bad =
            super::Config::read("testdata/config_bad.yml").expect("testdata/config_bad.yml");
        println!("config_bad={:?}", config_bad);
    }
}
