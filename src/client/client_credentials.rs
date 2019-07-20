use serde;

// Ozone response
// response={"access_token":"c4ddc2ae-7163-4c30-a0a8-c6b4f464a0d1","token_type":
// "Bearer","expires_in":3600}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Default,
)]
#[serde(deny_unknown_fields)]
pub struct ClientCredentialsGrant {
    /// REQUIRED. The access token issued by the authorization server.
    pub access_token: String,
    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType`
    /// parameter.
    pub token_type: String,
    /// RECOMMENDED. The lifetime in seconds of the access token. For example,
    /// the value 3600 denotes that the access token will expire in one hour
    /// from the time the response was generated. If omitted, the
    /// authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    // pub expires_in: u64,
    #[serde(deserialize_with = "from_millis")]
    pub expires_in: std::time::Duration,
}

// response="{\"access_token\":\"39f89ad2-6bca-497f-8f33-a7ecb41ba1d4\",\"token_type\":\"Bearer\",\"expires_in\":3600,\"scope\":\"openid accounts\",\"id_token\":\"eyJhbGciOiJub25lIn0.eyJzdWIiOiJhYWMtMWFiOGZkM2EtZGFlYi00NWVlLWI5ZGItZDA3ODEzMWIxN2FmIiwib3BlbmJhbmtpbmdfaW50ZW50X2lkIjoiYWFjLTFhYjhmZDNhLWRhZWItNDVlZS1iOWRiLWQwNzgxMzFiMTdhZiIsImlzcyI6Imh0dHBzOi8vbW9kZWxvYmFua2F1dGgyMDE4Lm8zYmFuay5jby51azo0MTAxIiwiYXVkIjoiM2ZjNTI4Y2YtZmM4OC00NmMyLTkzMTUtYThjZjg3MjQwNzVkIiwiaWF0IjoxNTYzMTkzNDk4LCJleHAiOjE1NjMxOTcwOTgsIm5vbmNlIjoiNWE2YjBkNzgzMmE5ZmI0ZjgwZjExNzBhIiwiY19oYXNoIjoiNlpzVlFMRkFKR29yVWhBTDlSNVdHZyIsInNfaGFzaCI6IjVmVVM0dzRzT1VvU0N5M2NUcnRuZnciLCJhY3IiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EifQ.\"}"
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Default,
)]
pub struct ExchangeToken {
    pub access_token:  String,
    pub refresh_token: Option<String>,
    pub scope:         String,
    pub id_token:      String,
    pub token_type:    String,
    pub expires_in:    u64, // relative seconds from now
    pub nonce:         Option<String>,
}

pub fn from_millis<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: u64 = serde::Deserialize::deserialize(deserializer)?;
    Ok(std::time::Duration::from_secs(s))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn test_deserialize_good() {
        let expected = super::ClientCredentialsGrant {
            access_token: "c875cc35-b712-4904-9ff1-9de9dc2b6014".to_string(),
            token_type:   "Bearer".to_string(),
            expires_in:   std::time::Duration::from_secs(3600),
        };
        let s = r#"
{
    "access_token": "c875cc35-b712-4904-9ff1-9de9dc2b6014",
    "token_type": "Bearer",
    "expires_in": 3600
}
        "#;
        let actual = serde_json::from_str::<super::ClientCredentialsGrant>(s).unwrap();

        assert_eq!(actual, expected);
    }
}
