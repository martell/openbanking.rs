use log::{debug, info};
use reqwest;
use std::io::Read;

pub mod openid_configuration;

pub fn fetch(config: super::config::Config) -> Result<(), Box<std::error::Error>> {
    let headers = super::client::defaults::headers();
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .default_headers(headers)
        .build()?;
    let openid_configuration = config.openid_configuration.clone();
    let url = openid_configuration.as_str();
    let request = client.get(url);
    debug!("request={:?}", request);

    let mut response = request.send().expect("request.send() failed");

    let mut response_buf = String::new();
    response
        .read_to_string(&mut response_buf)
        .expect("Failed to read response");
    debug!("response={}", response_buf);

    let openid_configuration: openid_configuration::OpenIDConfiguration =
        serde_json::from_str(response_buf.as_str()).unwrap();
    info!("openid_configuration={:?}", openid_configuration);

    let mut client = super::client::OpenBankingClient::new(config, openid_configuration)?;
    let account_requests_response = client.post_account_access_consents()?;
    client.post_account_access_consents_hybrid_flow(account_requests_response);

    Ok(())
}

// https://github.com/ramosbugs/openidconnect-rs/blob/master/src/discovery.rs
