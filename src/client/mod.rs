extern crate chrono;
extern crate reqwest;

use chrono::{DateTime, Utc};
use log::{debug, info};
use serde_json::json;
use std::io::Read;

use std::ops::Add;
use uuid::Uuid;

pub mod client_credentials;
pub mod certs;
pub mod defaults;

pub struct OpenBankingClient {
    pub config: super::config::Config,
    pub client: reqwest::Client,
    pub openid_configuration: super::oidcdiscovery::openid_configuration::OpenIDConfiguration,
}

impl Drop for OpenBankingClient {
    fn drop(&mut self) {
        println!("OpenBankingClient.drop");
    }
}

impl OpenBankingClient {
    pub fn new(
        config: super::config::Config,
        openid_configuration: super::oidcdiscovery::openid_configuration::OpenIDConfiguration,
    ) -> Result<OpenBankingClient, Box<std::error::Error>> {
        let transport_private = config.transport_private.clone();
        let transport_public = config.transport_public.clone();

        let mut identity_buf = Vec::new();
        // transport private and public key
        identity_buf.append(&mut transport_private.into_bytes());
        identity_buf.append(&mut transport_public.into_bytes());
        let pkcs12 = reqwest::Identity::from_pem(&identity_buf)?;
        debug!("pkcs12={:?}", pkcs12);

        let cert_issuing_ca = reqwest::Certificate::from_pem(certs::CERT_ISSUING_CA.as_bytes())?;
        debug!("cert_issuing_ca={:?}", cert_issuing_ca);
        let cert_root_ca = reqwest::Certificate::from_pem(certs::CERT_ROOT_CA.as_bytes())?;
        debug!("cert_root_ca={:?}", cert_root_ca);

        let headers = defaults::headers();
        let client = reqwest::Client::builder()
            .add_root_certificate(cert_issuing_ca)
            .add_root_certificate(cert_root_ca)
            .identity(pkcs12)
            .use_rustls_tls()
            .default_headers(headers)
            .build()?;

        Ok(OpenBankingClient {
            config: config,
            client: client,
            openid_configuration: openid_configuration,
        })
    }

    pub fn client_credentials(&mut self) -> Result<client_credentials::ClientCredentialsGrant, Box<std::error::Error>> {
        // https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
        let client_id = self.config.client_id.clone();
        let params = [
            ("grant_type", "client_credentials"),
            ("scope", "payments accounts"),
            // ("scope", "openid payments accounts fundsconfirmations")
            ("client_id", client_id.as_str()),
        ];
        debug!("params={:?}", params);

        // "https://matls.as.aspsp.ob.forgerock.financial/oauth2/access_token"
        let url = self.openid_configuration.token_endpoint.clone();
        let request = self.client.post(url.as_str()).form(&params);
        debug!("request={:?}", request);

        let mut response = String::new();
        request
            .send()
            .expect("client_credentials: request.send() failed")
            .read_to_string(&mut response)
            .expect("client_credentials: response.read_to_string() failed");
        info!("response={}", response);

        let client_credentials_grant: client_credentials::ClientCredentialsGrant =
            serde_json::from_str(response.as_str())?;
        // https://users.rust-lang.org/t/convert-std-time-systemtime-to-chrono-datetime-datetime/7684
        // let expires_in = std::time::Duration::from_secs(client_credentials_grant.expires_in);
        let expires_in = client_credentials_grant.expires_in;
        let toi: DateTime<chrono::offset::Utc> = std::time::SystemTime::now().into();
        let expiry: DateTime<chrono::offset::Utc> =
            std::time::SystemTime::now().add(expires_in).into();
        info!("client_credentials_grant={:?}", client_credentials_grant);
        info!("expires_in={:?}", expires_in);
        info!("toi={:?}", toi);
        info!("expiry={:?}", expiry);

        Ok(client_credentials_grant)
    }

    pub fn post_account_access_consents(&mut self) -> Result<(), Box<std::error::Error>> {
        let client_credentials_grant = self.client_credentials()?;

        // https://github.com/seanmonstar/reqwest/blob/master/examples/json_dynamic.rs
        let body = json!({
            "Data": {
                "Permissions": [
                    "ReadAccountsDetail",
                    "ReadBalances",
                    "ReadBeneficiariesDetail",
                    "ReadDirectDebits",
                    "ReadProducts",
                    "ReadStandingOrdersDetail",
                    "ReadTransactionsCredits",
                    "ReadTransactionsDebits",
                    "ReadTransactionsBasic",
                    "ReadTransactionsDetail",
                    "ReadStatementsBasic",
                    "ReadStatementsDetail",
                    "ReadPartyPSU",
                    "ReadBeneficiariesBasic",
                    "ReadAccountsBasic",
                    "ReadParty",
                    "ReadOffers",
                    "ReadScheduledPaymentsDetail"
                ],
                "ExpirationDateTime": "2050-12-12T00:00:00+00:00",
                "TransactionFromDateTime": "2017-05-03T00:00:00+00:00",
                "TransactionToDateTime": "2050-12-03T00:00:00+00:00"
            },
            "Risk": {}
        });

        let url = "https://modelobank2018.o3bank.co.uk:4501/open-banking/v3.1/aisp/account-access-consents";
        let mut headers = reqwest::header::HeaderMap::new();
        let header: [(&str, String); 8] = [
            (
                reqwest::header::ACCEPT.as_str(),
                "application/json".to_string(),
            ),
            (
                reqwest::header::CONTENT_TYPE.as_str(),
                "application/json".to_string(),
            ),
            (
                reqwest::header::AUTHORIZATION.as_str(),
                "Bearer ".to_string() + client_credentials_grant.access_token.as_str(),
            ),
            ("x-fapi-customer-ip-address", "104.25.212.99".to_string()),
            (
                "x-fapi-customer-last-logged-time",
                Utc::now().format("%a, %d %b %Y %T %Z").to_string(),
            ), // "Mon, 02 Jan 2006 15:04:05 MST"
            ("x-fapi-financial-id", self.config.financial_id.clone()),
            (
                "x-fapi-interaction-id",
                Uuid::new_v4().to_hyphenated().to_string(),
            ),
            ("x-idempotency-key", "FRESCO.21302.GFX.20".to_string()),
        ];
        for (key, val) in &header {
            headers.insert(*key, val.parse().unwrap());
        }
        let request = self
            .client
            .post(url)
            .body(body.to_string())
            .headers(headers);
        debug!("request={:?}", request);

        let mut response = String::new();
        request
            .send()
            .expect("post_account_access_consents: request.send() failed")
            .read_to_string(&mut response)
            .expect("post_account_access_consents: request.read_to_string() failed");
        info!("response={}", response);

        Ok(())
    }
}
