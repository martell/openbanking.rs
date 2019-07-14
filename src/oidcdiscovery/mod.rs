use log::debug;
use reqwest;
use serde;
use std::io::Read;

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
pub struct OpenIDConfiguration {
    pub request_parameter_supported:                      bool,
    pub claims_parameter_supported:                       bool,
    pub request_uri_parameter_supported:                  bool,
    pub introspection_endpoint:                           Option<String>,
    pub issuer:                                           String,
    pub authorization_endpoint:                           String,
    pub token_endpoint:                                   String,
    pub version:                                          Option<String>,
    pub userinfo_endpoint:                                Option<String>,
    pub jwks_uri:                                         String,
    pub registration_endpoint:                            String,
    pub require_request_uri_registration:                 bool,
    pub grant_types_supported:                            Vec<String>,
    pub scopes_supported:                                 Vec<String>,
    pub id_token_encryption_enc_values_supported:         Option<Vec<String>>,
    pub acr_values_supported:                             Option<Vec<String>>,
    pub request_object_encryption_enc_values_supported:   Option<Vec<String>>,
    pub claims_supported:                                 Vec<String>,
    pub claim_types_supported:                            Option<Vec<String>>,
    pub token_endpoint_auth_methods_supported:            Vec<String>,
    pub response_types_supported:                         Vec<String>,
    pub response_modes_supported:                         Option<Vec<String>>,
    pub id_token_encryption_alg_values_supported:         Option<Vec<String>>,
    pub subject_types_supported:                          Vec<String>,
    pub id_token_signing_alg_values_supported:            Vec<String>,
    pub request_object_signing_alg_values_supported:      Vec<String>,
    pub request_object_encryption_alg_values_supported:   Option<Vec<String>>,
    pub userinfo_signing_alg_values_supported:            Vec<String>,
    pub userinfo_encryption_enc_values_supported:         Option<Vec<String>>,
    pub userinfo_encryption_alg_values_supported:         Option<Vec<String>>,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
}

impl OpenIDConfiguration {
    pub fn fetch(config: super::config::Config) -> Result<Self, Box<std::error::Error>> {
        let default_headers = super::client::http::default_headers();
        let client =
            reqwest::Client::builder().use_rustls_tls().default_headers(default_headers).build()?;
        let openid_configuration = config.openid_configuration.clone();
        let url = openid_configuration.as_str();
        let request = client.get(url);

        let mut response = request.send().expect("request.send() failed");
        let mut response_buf = String::new();
        response.read_to_string(&mut response_buf).expect("Failed to read response");

        let openid_configuration = serde_json::from_str(response_buf.as_str()).unwrap();
        debug!("openid_configuration={:?}", openid_configuration);

        Ok(openid_configuration)
    }
}

// https://github.com/ramosbugs/openidconnect-rs/blob/master/src/discovery.rs

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize_good() {
        let openid_configuration_forgerock =
            include_str!("testdata/openid-configuration_forgerock.json");
        serde_json::from_str::<super::OpenIDConfiguration>(openid_configuration_forgerock).unwrap();

        let openid_configuration_ozone = include_str!("testdata/openid-configuration_ozone.json");
        serde_json::from_str::<super::OpenIDConfiguration>(openid_configuration_ozone).unwrap();
    }
}
