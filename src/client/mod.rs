use chrono;
use futures;
use log::{debug, error, info};
use reqwest;
use std::{io::Read, ops::Add};
use url::Url;

pub mod accounts;
pub mod claims;
pub mod client_credentials;

#[derive(Debug, Clone)]
pub struct OpenBankingClient {
    pub config:               super::config::Config,
    pub openid_configuration: super::oidcdiscovery::OpenIDConfiguration,
    pub client:               reqwest::Client, // reqwest::r#async::Client
}

impl OpenBankingClient {
    pub fn new(
        config: super::config::Config,
        openid_configuration: super::oidcdiscovery::OpenIDConfiguration,
    ) -> Result<Self, Box<std::error::Error>> {
        let client = super::http::new_client(config.clone())?;
        Ok(Self {
            config,
            openid_configuration,
            client,
        })
    }

    pub fn client_credentials(
        &self,
    ) -> Result<client_credentials::ClientCredentialsGrant, Box<std::error::Error>> {
        // https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
        let client_id = self.config.client_id.clone();
        let params = [
            ("grant_type", "client_credentials"),
            ("scope", "accounts"),
            ("client_id", client_id.as_str()),
        ];

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
        // let expires_in =
        // std::time::Duration::from_secs(client_credentials_grant.expires_in);
        let expires_in = client_credentials_grant.expires_in;
        let toi: chrono::DateTime<chrono::offset::Utc> = std::time::SystemTime::now().into();
        let expiry: chrono::DateTime<chrono::offset::Utc> =
            std::time::SystemTime::now().add(expires_in).into();
        info!(
            "client_credentials_grant={:?}, expires_in={:?}, toi={:?}, expiry={:?}",
            client_credentials_grant, expires_in, toi, expiry
        );

        Ok(client_credentials_grant)
    }

    pub fn post_account_access_consents(
        &self,
    ) -> Result<accounts::OBReadConsentResponse1, Box<std::error::Error>> {
        let client_credentials_grant = self.client_credentials()?;

        // https://github.com/seanmonstar/reqwest/blob/master/examples/json_dynamic.rs
        let body = serde_json::json!({
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

        // Read resource url from config.
        let url = "https://modelobank2018.o3bank.co.uk:4501/open-banking/v3.1/aisp/account-access-consents";
        let mut headers = reqwest::header::HeaderMap::new();
        let header: [(reqwest::header::HeaderName, String); 8] = [
            (reqwest::header::ACCEPT, "application/json".to_string()),
            (reqwest::header::CONTENT_TYPE, "application/json".to_string()),
            super::http::authorization(client_credentials_grant.access_token.clone()),
            super::http::x_fapi_financial_id(self.config.financial_id.clone()),
            super::http::x_fapi_customer_ip_address(),
            super::http::x_fapi_customer_last_logged_time(),
            super::http::x_idempotency_key(),
            super::http::x_fapi_interaction_id(),
        ];
        for (key, val) in &header {
            headers.insert(key, val.parse().unwrap());
        }

        let request = self.client.post(url).body(body.to_string()).headers(headers);
        debug!("request={:?}", request);

        let mut response = String::new();
        request
            .send()
            .expect("post_account_access_consents: request.send() failed")
            .read_to_string(&mut response)
            .expect("post_account_access_consents: request.read_to_string() failed");
        info!("response={}", response);

        let account_requests_response: accounts::OBReadConsentResponse1 =
            serde_json::from_str(response.as_str())?;

        Ok(account_requests_response)
    }

    pub fn post_account_access_consents_hybrid_flow(
        &self, account_requests_response: accounts::OBReadConsentResponse1,
    ) -> Result<String, Box<std::error::Error>> {
        let jwt = claims::JWT::new(
            self.config.client_id.clone(),
            self.config.kid.clone(),
            self.openid_configuration.issuer.clone(),
            self.config.request_object_signing_alg.clone(),
            account_requests_response.data.consent_id,
        )?;
        let request = jwt.to_string();

        let input = self.openid_configuration.authorization_endpoint.clone();
        let iter = &[
            ("request", request.as_str()),
            ("response_type", "code id_token"),
            ("client_id", self.config.client_id.as_str()),
            ("state", "state_accounts"),
            ("nonce", "5a6b0d7832a9fb4f80f1170a"),
            ("scope", "openid accounts"),
            ("redirect_uri", "https://127.0.0.1:8443/conformancesuite/callback"),
        ];

        // https://modelobankauth2018.o3bank.co.uk:4101/auth?client_id=3fc528cf-fc88-46c2-9315-a8cf8724075d&redirect_uri=https%3A%2F%2F127.0.0.1%3A8443%2Fconformancesuite%2Fcallback&request=eyJhbGciOiJQUzI1NiIsImtpZCI6IlF1RllCUkpuV2RJNl9OSEZnYW11WE5yNVIyMCIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJodHRwczovL21vZGVsb2JhbmthdXRoMjAxOC5vM2JhbmsuY28udWs6NDEwMSIsImNsYWltcyI6eyJpZF90b2tlbiI6eyJvcGVuYmFua2luZ19pbnRlbnRfaWQiOnsiZXNzZW50aWFsIjp0cnVlLCJ2YWx1ZSI6ImFhYy00MDhiMzlhNS1hOTYwLTQ2MWEtOWE3MC1iMjc2ZjFmOGYwYjcifX19LCJjbGllbnRfaWQiOiIzZmM1MjhjZi1mYzg4LTQ2YzItOTMxNS1hOGNmODcyNDA3NWQiLCJleHAiOjE1NjE5ODg3ODcsImlhdCI6MTU2MTk4Njk4NywiaXNzIjoiM2ZjNTI4Y2YtZmM4OC00NmMyLTkzMTUtYThjZjg3MjQwNzVkIiwianRpIjoiY2ZhNjQ3NjUtOWNlNS00NGNhLWE5MDctODc5M2VhZDRmYmVmIiwibm9uY2UiOiJjZmE2NDc2NS05Y2U1LTQ0Y2EtYTkwNy04NzkzZWFkNGZiZWYiLCJyZWRpcmVjdF91cmkiOiJodHRwczovLzEyNy4wLjAuMTo4NDQzL2NvbmZvcm1hbmNlc3VpdGUvY2FsbGJhY2siLCJyZXNwb25zZV90eXBlIjoiY29kZSBpZF90b2tlbiIsInNjb3BlIjoib3BlbmlkIGFjY291bnRzIiwic3RhdGUiOiJhY2NvdW50VG9rZW4wMDAyIiwic3ViIjoiM2ZjNTI4Y2YtZmM4OC00NmMyLTkzMTUtYThjZjg3MjQwNzVkIn0.d4CWmroLhicUOqAW3zU4ybdPPauWE5R-2ssDrF_-ujnyF8mKrSgg2McXLZJhMXrCLmTOmWyzKpe_0KMK2wnHixJqcHXt2XDZybet0GPJUcNRhz2eRkIWTCSrBPDvJC0YIOw1ggjrB2oxIhefClTt_HM0s_ohhvyRrbgpdRZN6NAaFCQ2-lglEE_Mm9Rc1FYIh4F8J8x0hEPH842bWc76PqpR2Wmt8rEydnrKLukEAjr3_4wuovBll_4VkiWWZiKaD9lu9wJou4t0I8rjAFI6ypYMeuCkvluP8euEKpVnFf8X57xQqvBuxeWE8j7PpGQoMrYY87oludHVBJSu3Ammgw&response_type=code+id_token&scope=openid+accounts&state=accountToken0002
        let url = Url::parse_with_params(input.as_str(), iter)?;
        // "https://modelobankauth2018.o3bank.co.uk:4101/auth?request=eyJhbGciOiJQUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IlF1RllCUkpuV2RJNl9OSEZnYW11WE5yNVIyMCJ9.eyJpc3MiOiIzZmM1MjhjZi1mYzg4LTQ2YzItOTMxNS1hOGNmODcyNDA3NWQiLCJzdWIiOiIzZmM1MjhjZi1mYzg4LTQ2YzItOTMxNS1hOGNmODcyNDA3NWQiLCJhdWQiOiJodHRwczovL21vZGVsb2JhbmthdXRoMjAxOC5vM2JhbmsuY28udWs6NDEwMSIsImV4cCI6MTU2MTk5MTU1MywiaWF0IjoxNTYxOTg5ODEzLCJqdGkiOiI5M2JkYjczZS00Y2QyLTQ2YjMtOWIzZC1jNTZiMGQ3ZDllY2UiLCJzY29wZSI6Im9wZW5pZCBhY2NvdW50cyIsImNsYWltcyI6eyJpZF90b2tlbiI6eyJhY3IiOnsidmFsdWUiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EiLCJlc3NlbnRpYWwiOnRydWV9LCJvcGVuYmFua2luZ19pbnRlbnRfaWQiOnsidmFsdWUiOiJhYWMtZjJlM2JhMTMtYWI5OC00OTExLWEyNzctMzgzMGM5YzQ3OGY1IiwiZXNzZW50aWFsIjp0cnVlfX0sInVzZXJpbmZvIjp7Im9wZW5iYW5raW5nX2ludGVudF9pZCI6eyJ2YWx1ZSI6ImFhYy1mMmUzYmExMy1hYjk4LTQ5MTEtYTI3Ny0zODMwYzljNDc4ZjUiLCJlc3NlbnRpYWwiOnRydWV9fX0sInJlZGlyZWN0X3VyaSI6Imh0dHBzOi8vMTI3LjAuMC4xOjg0NDMvY29uZm9ybWFuY2VzdWl0ZS9jYWxsYmFjayIsInN0YXRlIjoic3RhdGVfYWNjb3VudHMiLCJub25jZSI6IjVhNmIwZDc4MzJhOWZiNGY4MGYxMTcwYSIsImNsaWVudF9pZCI6IjNmYzUyOGNmLWZjODgtNDZjMi05MzE1LWE4Y2Y4NzI0MDc1ZCIsInJlc3BvbnNlX3R5cGUiOiJjb2RlIGlkX3Rva2VuIn0.bNY_6Z4sdNISIFPBcTwG3G3zXelUgU79P3Cd85qws0jcvGkZS518JuxgretNmtCAecn5KBDTWrJnCxPJesH_JRjY-_SJMVrI_chrvFLxI3oW_pVB0HfKj26hvNVyy9YOPtTW5xH9R3b3kJZHi9wbGNTugp3mXkPkAa80p0TBW1uZVVOr25SKs6hpOJRX4u24k4gzktJ3WvcH7vzN4IPjUDrX_XhCJ_RITRMKGmGxjplGYpojDTTU0ekZFfviLe46o75LXSzBpEK_V5eyGkh1pAZI9grDKkkT6L0yuGJ_aiyqMAdjzlIhRngi7wfEVkHc7wS5Zd-Jjgi3wDFyHm8Kmw&response_type=code+id_token&client_id=3fc528cf-fc88-46c2-9315-a8cf8724075d&state=state_accounts&nonce=5a6b0d7832a9fb4f80f1170a&scope=openid+accounts&redirect_uri=https%3A%2F%2F127.0.0.1%3A8443%2Fconformancesuite%2Fcallback"
        Ok(url.into_string())
    }

    // Exchange `code` for `access_token`
    pub fn post_token_exchange(
        &self, code: String, scope: Option<String>, id_token: String, state: String,
    ) -> Box<futures::Future<Item = String, Error = Box<dyn std::error::Error>>> {
        info!("code={:?}, scope={:?}, id_token={:?}, state={:?}", code, scope, id_token, state);

        let client_id = self.config.client_id.clone();
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", "https://127.0.0.1:8443/conformancesuite/callback"),
            ("client_id", client_id.as_str()),
        ];

        use futures::future::Future;

        // https://github.com/seanmonstar/reqwest/blob/master/examples/async.rs
        // https://github.com/actix/actix-web/blob/master/examples/client.rs
        // https://github.com/seanmonstar/reqwest/blob/564a08f23041edb5e7384e4a4d90accdef6b06c9/tests/async.rs#L95
        // https://github.com/seanmonstar/reqwest/blob/564a08f23041edb5e7384e4a4d90accdef6b06c9/tests/async.rs#L59
        let client = match super::http::new_async_client(self.config.clone()) {
            Ok(client) => client,
            Err(error) => return Box::new(futures::future::err(error)),
        };
        let url = self.openid_configuration.token_endpoint.clone();
        let future = client
            .post(url.as_str())
            .form(&params)
            .send()
            .and_then(|mut res| {
                use futures::stream::Stream;

                println!("status={:?}", res.status());
                let body = std::mem::replace(res.body_mut(), reqwest::r#async::Decoder::empty());
                body.concat2()
            })
            .map_err(|error| {
                error!("outer send - error={:?}", error);
                error.into()
            })
            .map(|body| {
                let v = body.to_vec();
                // response="{\"error\":\"invalid_request\",\"error_description\":\"
                // authorization code invalid\"}"
                let response = String::from_utf8_lossy(&v).to_string();
                info!("outer map - response={:?}", response);
                response
            })
            .and_then(|response: String| {
                // response="{\"access_token\":\"6a823ad0-4643-4b02-b057-83d20651ea05\",\"token_type\":\"Bearer\",\"expires_in\":3600,\"scope\":\"openid accounts\",\"id_token\":\"eyJhbGciOiJub25lIn0.eyJzdWIiOiJhYWMtODMyZTEzM2YtOTE5My00NTNlLWE3NGQtZTRjM2NmN2YyODQ2Iiwib3BlbmJhbmtpbmdfaW50ZW50X2lkIjoiYWFjLTgzMmUxMzNmLTkxOTMtNDUzZS1hNzRkLWU0YzNjZjdmMjg0NiIsImlzcyI6Imh0dHBzOi8vbW9kZWxvYmFua2F1dGgyMDE4Lm8zYmFuay5jby51azo0MTAxIiwiYXVkIjoiM2ZjNTI4Y2YtZmM4OC00NmMyLTkzMTUtYThjZjg3MjQwNzVkIiwiaWF0IjoxNTYzMTkzODUzLCJleHAiOjE1NjMxOTc0NTMsIm5vbmNlIjoiNWE2YjBkNzgzMmE5ZmI0ZjgwZjExNzBhIiwiY19oYXNoIjoiZmdWSXJ4aDBhMkZqdXI2d0wtOE9XdyIsInNfaGFzaCI6IjVmVVM0dzRzT1VvU0N5M2NUcnRuZnciLCJhY3IiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EifQ.\"}"
                let json: Result<client_credentials::ExchangeToken, _> =
                    serde_json::from_str(response.as_str());
                match json {
                    Ok(json) => futures::future::ok(json),
                    Err(error) => futures::future::err(error.into()),
                }
                // info!("outer convert - response={:?}", response);
                // response.json::<client_credentials::ExchangeToken>().map_err(|error| {
                //     error!("inner convert - error={:?}", error);
                //     error.into()
                // })
            })
            .map_err(|error: Box<dyn std::error::Error>| {
                error!("outer convert - error={:?}", error);
                error.into()
            })
            .map(|response| response.access_token.clone())
            .and_then(|response: String| futures::future::ok(response));

        Box::new(future)
    }
}
