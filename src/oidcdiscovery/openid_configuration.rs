use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OpenIDConfiguration {
    pub request_parameter_supported: bool,
    pub claims_parameter_supported: bool,
    pub request_uri_parameter_supported: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let openid_configuration_forgerock =
            include_str!("testdata/openid-configuration_forgerock.json");
        let openid_configuration_ozone = include_str!("testdata/openid-configuration_ozone.json");
        serde_json::from_str::<OpenIDConfiguration>(openid_configuration_forgerock).unwrap();
        serde_json::from_str::<OpenIDConfiguration>(openid_configuration_ozone).unwrap();
    }
}
