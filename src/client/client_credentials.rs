extern crate chrono;
extern crate reqwest;

use serde::{Deserialize, Serialize};

// Ozone response
// response={"access_token":"c4ddc2ae-7163-4c30-a0a8-c6b4f464a0d1","token_type":"Bearer","expires_in":3600}
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ClientCredentialsGrant {
    ///
    /// REQUIRED. The access token issued by the authorization server.
    ///
    pub access_token: String,
    ///
    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType` parameter.
    ///
    pub token_type: String,
    ///
    /// RECOMMENDED. The lifetime in seconds of the access token. For example, the value 3600
    /// denotes that the access token will expire in one hour from the time the response was
    /// generated. If omitted, the authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    ///
    // pub expires_in: u64,
    #[serde(deserialize_with = "from_millis")]
    pub expires_in: std::time::Duration,
}

fn from_millis<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: u64 = Deserialize::deserialize(deserializer)?;
    Ok(std::time::Duration::from_secs(s))
}

impl Drop for ClientCredentialsGrant {
    fn drop(&mut self) {
        println!("ClientCredentialsGrant.drop");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_deserialize() {
        let expected = ClientCredentialsGrant {
            access_token: "c875cc35-b712-4904-9ff1-9de9dc2b6014".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: std::time::Duration::from_secs(3600),
        };
        let json = r#"{"access_token":"c875cc35-b712-4904-9ff1-9de9dc2b6014","token_type":"Bearer","expires_in":3600}"#;
        let actual = serde_json::from_str::<ClientCredentialsGrant>(json).unwrap();
        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }
}
