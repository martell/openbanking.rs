extern crate reqwest;

use log::{debug, info};

use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct OpenIDConfiguration {
    pub request_parameter_supported: bool,
    pub claims_parameter_supported: bool,
    pub request_uri_parameter_supported: bool,
    // #[serde(
    //     default,
    //     deserialize_with = "deserialize_option_or_none",
    //     skip_serializing_if = "Option::is_none"
    // )]
    pub introspection_endpoint: Option<String>,
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub version: Option<String>,
    pub userinfo_endpoint: Option<String>,
    pub jwks_uri: String,
    pub registration_endpoint: String,
    pub require_request_uri_registration: bool,
    pub grant_types_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub id_token_encryption_enc_values_supported: Option<Vec<String>>,
    pub acr_values_supported: Option<Vec<String>>,
    pub request_object_encryption_enc_values_supported: Option<Vec<String>>,
    pub claims_supported: Vec<String>,
    pub claim_types_supported: Option<Vec<String>>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub response_modes_supported: Option<Vec<String>>,
    pub id_token_encryption_alg_values_supported: Option<Vec<String>>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub request_object_signing_alg_values_supported: Vec<String>,
    pub request_object_encryption_alg_values_supported: Option<Vec<String>>,
    pub userinfo_signing_alg_values_supported: Vec<String>,
    pub userinfo_encryption_enc_values_supported: Option<Vec<String>>,
    pub userinfo_encryption_alg_values_supported: Option<Vec<String>>,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
}

pub fn fetch(config: super::config::Config) -> Result<(), Box<std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    {
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "banaio-openbankingforgerock/0.1.0 (https://github.com/banaio/openbanking.rs)",
            ),
        );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("no-cache"),
        );
        debug!("headers={:?}", headers);
    }

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .default_headers(headers)
        .build()?;

    let url = "https://modelobankauth2018.o3bank.co.uk:4101/.well-known/openid-configuration";
    let request = client.get(url);
    debug!("request={:?}", request);

    let mut response = request.send().expect("request.send() failed");

    let mut response_buf = String::new();
    response
        .read_to_string(&mut response_buf)
        .expect("Failed to read response");
    debug!("response={}", response_buf);

    let openidconfiguration: OpenIDConfiguration =
        serde_json::from_str(response_buf.as_str()).unwrap();
    info!("openidconfiguration={:?}", openidconfiguration);

    super::client::main(config, openidconfiguration)?;

    Ok(())
}

// https://github.com/ramosbugs/openidconnect-rs/blob/master/src/discovery.rs
