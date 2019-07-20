use chrono;
use log::debug;
use reqwest;
use uuid;

pub mod certs;

pub struct ClientBuilderParams {
    pub cert_issuing_ca: reqwest::Certificate,
    pub cert_root_ca:    reqwest::Certificate,
    pub pkcs12:          reqwest::Identity,
    pub default_headers: reqwest::header::HeaderMap,
}

pub fn client_builder_params(
    config: crate::config::Config,
) -> Result<ClientBuilderParams, Box<std::error::Error>> {
    let transport_private = config.transport_private.clone();
    let transport_public = config.transport_public.clone();

    let mut identity_buf = Vec::new();
    // transport private and public key
    identity_buf.append(&mut transport_private.into_bytes());
    identity_buf.append(&mut transport_public.into_bytes());
    let pkcs12 = reqwest::Identity::from_pem(&identity_buf)?;

    let cert_issuing_ca = reqwest::Certificate::from_pem(certs::CERT_ISSUING_CA.as_bytes())?;
    let cert_root_ca = reqwest::Certificate::from_pem(certs::CERT_ROOT_CA.as_bytes())?;

    let default_headers = default_headers();
    let params = ClientBuilderParams {
        cert_issuing_ca,
        cert_root_ca,
        pkcs12,
        default_headers,
    };
    Ok(params)
}

pub fn new_async_client(
    config: crate::config::Config,
) -> Result<reqwest::r#async::Client, Box<std::error::Error>> {
    let params = client_builder_params(config)?;
    let builder = reqwest::r#async::Client::builder();
    let client = builder
        .add_root_certificate(params.cert_issuing_ca)
        .add_root_certificate(params.cert_root_ca)
        .identity(params.pkcs12)
        .use_rustls_tls()
        .default_headers(params.default_headers)
        .build()?;
    debug!("client={:?}", client);

    Ok(client)
}

pub fn new_client(
    config: crate::config::Config,
) -> Result<reqwest::Client, Box<std::error::Error>> {
    let params = client_builder_params(config)?;
    let builder = reqwest::Client::builder();
    let client = builder
        .add_root_certificate(params.cert_issuing_ca)
        .add_root_certificate(params.cert_root_ca)
        .identity(params.pkcs12)
        .use_rustls_tls()
        .default_headers(params.default_headers)
        .build()?;
    debug!("client={:?}", client);

    Ok(client)
}

// move to a utils module?
pub fn new_id() -> String {
    uuid::Uuid::new_v4().to_hyphenated().to_string()
}

pub fn authorization(access_token: String) -> (reqwest::header::HeaderName, String) {
    (reqwest::header::AUTHORIZATION, "Bearer ".to_string() + access_token.as_str())
}

pub fn x_fapi_financial_id(financial_id: String) -> (reqwest::header::HeaderName, String) {
    (reqwest::header::HeaderName::from_static("x-fapi-financial-id"), financial_id.clone())
}

pub fn x_fapi_interaction_id() -> (reqwest::header::HeaderName, String) {
    (reqwest::header::HeaderName::from_static("x-fapi-interaction-id"), new_id())
}

pub fn x_fapi_customer_last_logged_time() -> (reqwest::header::HeaderName, String) {
    // "Mon, 02 Jan 2006 15:04:05 MST"
    (
        reqwest::header::HeaderName::from_static("x-fapi-customer-last-logged-time"),
        chrono::Utc::now().format("%a, %d %b %Y %T %Z").to_string(),
    )
}

pub fn x_fapi_customer_ip_address() -> (reqwest::header::HeaderName, String) {
    (
        reqwest::header::HeaderName::from_static("x-fapi-customer-ip-address"),
        "104.25.212.99".to_string(),
    )
}

pub fn x_idempotency_key() -> (reqwest::header::HeaderName, String) {
    (
        reqwest::header::HeaderName::from_static("x-idempotency-key"),
        "FRESCO.21302.GFX.20".to_string(),
    )
}

pub fn default_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    // headers.insert(
    //     reqwest::header::USER_AGENT,
    //     reqwest::header::HeaderValue::from_static(
    //         "banaio-openbankingforgerock/0.1.0 (https://github.com/banaio/openbanking.rs)",
    //     ),
    // );
    headers.insert(
        reqwest::header::USER_AGENT,
        format!(
            "reqwest/{} banaio-openbankingforgerock/{} (https://github.com/banaio/openbanking.rs)",
            "0.9.18",
            env!("CARGO_PKG_VERSION")
        )
        .as_str()
        .parse()
        .unwrap(),
    );
    headers.insert(
        reqwest::header::CACHE_CONTROL,
        reqwest::header::HeaderValue::from_static("no-cache"),
    );
    debug!("default_headers={:?}", headers);

    headers
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_headers() {
        let mut expected = reqwest::header::HeaderMap::new();
        expected.insert(
            "user-agent",
            "reqwest/0.9.18 banaio-openbankingforgerock/0.1.0 \
             (https://github.com/banaio/openbanking.rs)"
                .parse()
                .unwrap(),
        );
        expected.insert("cache-control", "no-cache".parse().unwrap());

        let actual = super::default_headers();

        assert_eq!(actual, expected);
    }
}
