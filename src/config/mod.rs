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
        // Note this is required to convert from `serde_yaml::Error` to
        // `std::error::Error`. See this blog post for more information:
        // * https://blog.burntsushi.net/rust-error-handling/#the-from-trait
        // * https://blog.burntsushi.net/rust-error-handling/#the-real-try-macro-operator
        //
        // The type of `serde_yaml::from_str(value)?` is `result::Result<T,
        // serde_yaml::Error>` but we are returning `Box<std::error::Error>`
        // this is because any type that impls std::error::Error, can be converted to a
        // trait object `Box<std::error::Error>` because the following exists:
        //
        // `impl<'a, E: Error + 'a> From<E> for Box<Error + 'a>`
        serde_yaml::from_str(value).map_err(std::convert::From::from)
    }
}

impl std::str::FromStr for Config {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Note this is required to convert from `serde_yaml::Error` to
        // `std::error::Error`. See this blog post for more information:
        // * https://blog.burntsushi.net/rust-error-handling/#the-from-trait
        // * https://blog.burntsushi.net/rust-error-handling/#the-real-try-macro-operator
        //
        // The type of `serde_yaml::from_str(value)?` is `result::Result<T,
        // serde_yaml::Error>` but we are returning `Box<std::error::Error>`
        // this is because any type that impls std::error::Error, can be converted to a
        // trait object `Box<std::error::Error>` because the following exists:
        //
        // `impl<'a, E: Error + 'a> From<E> for Box<Error + 'a>`
        serde_yaml::from_str(s).map_err(std::convert::From::from)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    // Use: https://doc.rust-lang.org/stable/std/panic/fn.catch_unwind.html

    #[test]
    fn test_read_config_good() {
        let actual = super::Config::read("src/config/testdata/config_good.yml").unwrap();
        let expected = super::Config {
            ssid:                       "<some_software_statement_id>".into(),
            organisation_id:            "<some_organisation_id>".into(),
            ssa:                        "<some_software_statement_header>.\
                                         <some_software_statement_payload>.\
                                         <some_software_statement_signature>"
                .into(),
            kid:                        "<some_kid>".into(),
            client_id:                  "<some_client_id>".into(),
            request_object_signing_alg: "PS256".into(),
            encryption_private:         "testdata/keys/encryption_private.key".into(),
            encryption_public:          "testdata/keys/encryption_public.pem".into(),
            signature_private:          "testdata/keys/signature_private.key".into(),
            signature_public:           "testdata/keys/signature_public.pem".into(),
            transport_private:          "testdata/keys/transport_private.key".into(),
            transport_public:           "testdata/keys/transport_public.pem".into(),
            register_response:          "testdata/register_response.json".into(),
            openid_configuration:       "<openid_configuration>".into(),
            financial_id:               "<financial_id>".into(),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_read_config_bad() {
        let _ = super::Config::read("testdata/config_bad.yml").expect("testdata/config_bad.yml");
    }
}
