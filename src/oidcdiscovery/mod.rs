use log::debug;
use reqwest;
use serde;

// https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
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
// #[serde(deny_unknown_fields)]
pub struct OpenIDConfiguration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    pub response_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameter_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_parameter_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_uri_parameter_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_request_uri_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_encryption_enc_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_encryption_enc_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_encryption_enc_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_encryption_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
}

impl OpenIDConfiguration {
    pub fn fetch(config: super::config::Config) -> Result<Self, Box<std::error::Error>> {
        use std::io::Read;

        let default_headers = super::http::default_headers();
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
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn test_deserialize_good() {
        let openid_configuration_forgerock = serde_json::from_str::<super::OpenIDConfiguration>(
            include_str!("testdata/openid-configuration_forgerock.json"),
        )
        .unwrap();
        let openid_configuration_ozone = serde_json::from_str::<super::OpenIDConfiguration>(
            include_str!("testdata/openid-configuration_ozone.json"),
        )
        .unwrap();
        let openid_configuration_ozone_v2 = serde_json::from_str::<super::OpenIDConfiguration>(
            include_str!("testdata/openid-configuration_ozone_v2.json"),
        )
        .unwrap();

        // println!("openid_configuration_forgerock={:#?}",
        // serde_json::to_string_pretty(&openid_configuration_forgerock).unwrap());
        // println!("openid_configuration_ozone={:#?}",
        // serde_json::to_string_pretty(&openid_configuration_ozone).unwrap());
        // println!("openid_configuration_ozone_v2={:#?}",
        // serde_json::to_string_pretty(&openid_configuration_ozone_v2).unwrap());

        assert_eq!(
            openid_configuration_forgerock.issuer,
            "https://as.aspsp.ob.forgerock.financial/oauth2"
        );
        assert_eq!(
            openid_configuration_ozone.issuer,
            "https://modelobankauth2018.o3bank.co.uk:4101"
        );
        assert_eq!(openid_configuration_ozone_v2.issuer, "https://ob19-auth1-ui.o3bank.co.uk");
    }
}
