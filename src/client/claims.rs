use biscuit;
use chrono;
use serde;

// {
//     "id_token": {
//         "acr": {
//             "value": "urn:openbanking:psd2:sca",
//             "essential": true
//         },
//         "openbanking_intent_id": {
//             "value": "<consent_id>",
//             "essential": true
//         }
//     },
//     "userinfo": {
//         "openbanking_intent_id": {
//             "value": "<consent_id>",
//             "essential": true
//         }
//     }
// }
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(deny_unknown_fields)]
pub struct Claims {
    pub id_token: IdToken,
    pub userinfo: UserInfo,
}

impl Claims {
    pub fn new(consent_id: String) -> Claims {
        Claims {
            id_token: IdToken {
                acr:                   Acr {
                    ..Default::default()
                },
                openbanking_intent_id: OpenbankingIntentId {
                    value:     consent_id.clone(),
                    essential: true,
                },
            },
            userinfo: UserInfo {
                openbanking_intent_id: OpenbankingIntentId {
                    value:     consent_id.clone(),
                    essential: true,
                },
            },
        }
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(deny_unknown_fields)]
pub struct IdToken {
    pub acr:                   Acr,
    pub openbanking_intent_id: OpenbankingIntentId,
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(deny_unknown_fields)]
pub struct UserInfo {
    pub openbanking_intent_id: OpenbankingIntentId,
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(deny_unknown_fields)]
pub struct Acr {
    pub value:     String,
    pub essential: bool,
}

impl Default for Acr {
    fn default() -> Acr {
        Acr {
            value:     "urn:openbanking:psd2:sca".into(),
            essential: true,
        }
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(deny_unknown_fields)]
pub struct OpenbankingIntentId {
    pub value:     String,
    pub essential: bool,
}

impl Default for OpenbankingIntentId {
    fn default() -> OpenbankingIntentId {
        OpenbankingIntentId {
            essential: true,
            value:     "".into(),
        }
    }
}

// {
//   "aud": "https://modelobankauth2018.o3bank.co.uk:4101",
//   "claims": {
//     "id_token": {
//       "openbanking_intent_id": {
//         "essential": true,
//         "value": "aac-408b39a5-a960-461a-9a70-b276f1f8f0b7"
//       }
//     }
//   },
//   "client_id": "3fc528cf-fc88-46c2-9315-a8cf8724075d",
//   "exp": 1561988787,
//   "iat": 1561986987,
//   "iss": "3fc528cf-fc88-46c2-9315-a8cf8724075d",
//   "jti": "cfa64765-9ce5-44ca-a907-8793ead4fbef",
//   "nonce": "cfa64765-9ce5-44ca-a907-8793ead4fbef",
//   "redirect_uri": "https://127.0.0.1:8443/conformancesuite/callback",
//   "response_type": "code id_token",
//   "scope": "openid accounts",
//   "state": "accountToken0002",
//   "sub": "3fc528cf-fc88-46c2-9315-a8cf8724075d"
// }
// Define our own private claims
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(deny_unknown_fields)]
pub struct PrivateClaims {
    pub scope:         String,
    pub claims:        Claims,
    pub redirect_uri:  String,
    pub state:         String,
    pub nonce:         String,
    pub client_id:     String,
    pub response_type: String,
}

impl PrivateClaims {
    pub fn new(client_id: String, consent_id: String) -> Self {
        PrivateClaims {
            scope:         "openid accounts".into(),
            claims:        Claims::new(consent_id),
            redirect_uri:  "https://127.0.0.1:8443/conformancesuite/callback".into(),
            state:         "state_accounts".into(),
            nonce:         "5a6b0d7832a9fb4f80f1170a".into(),
            client_id:     client_id.clone(),
            response_type: "code id_token".into(),
        }
    }
}

pub struct JWT(pub String);

impl std::fmt::Display for JWT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type ClaimsSet = biscuit::ClaimsSet<PrivateClaims>;
pub type JWS = biscuit::jws::Compact<ClaimsSet, biscuit::Empty>;

impl JWT {
    pub fn payload(client_id: String, issuer: String, private: PrivateClaims) -> ClaimsSet {
        use biscuit::{SingleOrMultiple, StringOrUri};

        let expiry =
            Some(std::convert::From::from(chrono::Utc::now() + chrono::Duration::minutes(29)));
        let issued_at = Some(std::convert::From::from(chrono::Utc::now()));
        let id = Some(crate::http::new_id());
        let registered = biscuit::RegisteredClaims {
            issuer: Some(StringOrUri::String(client_id.clone())),
            subject: Some(StringOrUri::String(client_id.clone())),
            audience: Some(SingleOrMultiple::Single(StringOrUri::String(issuer.clone()))),
            expiry,
            issued_at,
            id,
            ..Default::default()
        };

        ClaimsSet {
            registered,
            private,
        }
    }

    pub fn jwt(
        client_id: String, kid: String, issuer: String, request_object_signing_alg: String,
        consent_id: String,
    ) -> JWS {
        let private = PrivateClaims::new(client_id.clone(), consent_id.clone());
        let payload = JWT::payload(client_id.clone(), issuer.clone(), private);
        let algorithm = match request_object_signing_alg.as_str() {
            "PS256" => biscuit::jwa::SignatureAlgorithm::PS256,
            "RS256" => biscuit::jwa::SignatureAlgorithm::RS256,
            _ => biscuit::jwa::SignatureAlgorithm::PS256,
        };
        let jwt = biscuit::JWT::new_decoded(
            From::from(biscuit::jws::RegisteredHeader {
                algorithm,
                key_id: Some(kid.clone()),
                ..Default::default()
            }),
            payload.clone(),
        );

        jwt
    }

    // forgerock `request`:
    // eyJhbGciOiJSUzI1NiIsImtpZCI6ImQ2YzNmNDlkLTcxMTItNGM1Yy05YzlkLTg0OTI2ZTk5MmM3NCIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJodHRwczovL21hdGxzLmFzLmFzcHNwLm9iLmZvcmdlcm9jay5maW5hbmNpYWwvb2F1dGgyL29wZW5iYW5raW5nIiwiY2xhaW1zIjp7ImlkX3Rva2VuIjp7ImFjciI6eyJlc3NlbnRpYWwiOnRydWUsInZhbHVlIjoidXJuOm9wZW5iYW5raW5nOnBzZDI6c2NhIn0sIm9wZW5iYW5raW5nX2ludGVudF9pZCI6eyJlc3NlbnRpYWwiOnRydWUsInZhbHVlIjoiQWJjM2UwOGJjLTcyYzUtNGUzMy1hYjYwLThiZDlhZjhhZGMxNiJ9fSwidXNlcmluZm8iOnsib3BlbmJhbmtpbmdfaW50ZW50X2lkIjp7ImVzc2VudGlhbCI6dHJ1ZSwidmFsdWUiOiJBYmMzZTA4YmMtNzJjNS00ZTMzLWFiNjAtOGJkOWFmOGFkYzE2In19fSwiY2xpZW50X2lkIjoiNTRmNjQzMDktNDMzZC00NjEwLTk1ZDItNjNkMmY1MjUzNDEyIiwiZXhwIjoxNTQwMTk3OTk5LCJpYXQiOjE1NDAxOTc4NzksImlzcyI6IjU0ZjY0MzA5LTQzM2QtNDYxMC05NWQyLTYzZDJmNTI1MzQxMiIsImp0aSI6IjJmODMyMzJjLTA0NmUtNGIyMC05NTc4LWRmMTljOTdhZTNmOSIsIm5vbmNlIjoiNWE2YjBkNzgzMmE5ZmI0ZjgwZjExNzBhIiwicmVkaXJlY3RfdXJpIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwL29wZW5iYW5raW5nL2JhbmFpby9mb3JnZXJvY2siLCJyZXNwb25zZV90eXBlIjoiY29kZSBpZF90b2tlbiIsInNjb3BlIjoiYWNjb3VudHMgb3BlbmlkIiwic3RhdGUiOiI1YTZiMGQ3ODMyYTlmYjRmODBmMTE3MGEifQ.KuTvvOC2yz5idjUVH6I7aZlHj0jGuR06zJlNny8D01XoHvm0xA27YXyIwsQO-q0MlMDErBzzU8WMZ3-6wJxWth4thPmSu5zzVAYo7ZWEUDHhlq7YWZkATRintLv0PqUlx_h8r8N2tmtm0UWE2VtxKdRQN1jSD7_kjsw7w_vaP_OwvOA8lGEjU30JW4HxHLfxyeIjHxsTY_dlSiHvWwdmqlwEW9DQJtAYHGboJkX6GBXqV5zEHD4UdtjRYIkyPDAgHqt5smiEzMcuGwJoD2v4vSBEmpEdnmAANgPFxKhNsyNhm7HQXaL6vRLuasgrg7JW9F8iWvw-4BlASAcoBiwKCg
    pub fn new(
        client_id: String, kid: String, issuer: String, request_object_signing_alg: String,
        consent_id: String,
    ) -> Result<Self, Box<std::error::Error>> {
        let jwt = JWT::jwt(
            client_id.clone(),
            kid.clone(),
            issuer.clone(),
            request_object_signing_alg.clone(),
            consent_id.clone(),
        );

        // let signing_secret =
        //     biscuit::jws::Secret::rsa_keypair_from_file("src/client/private_key.der"
        // ).unwrap();
        let signing_secret = biscuit::jws::Secret::rsa_keypair_from_file(
            "keys/config_tls_client_auth_ps256_ozone/private_key.der",
        )?;
        // let signing_secret =
        // biscuit::jws::Secret::Bytes("secret".to_string().into_bytes());
        let compact_encoded_jwt = jwt.into_encoded(&signing_secret)?;
        let jwt = compact_encoded_jwt.encoded()?.to_string();
        // let jwt = compact_encoded_jwt.unwrap_encoded().to_string();

        Ok(JWT(jwt))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn test_private_claims() {
        let client_id: String = "stub_client_id".into();
        let consent_id: String = "stub_consent_id".into();
        let expected = serde_json::json!({
            "scope":         "openid accounts",
            "claims":        super::Claims::new(consent_id.clone()),
            "redirect_uri":  "https://127.0.0.1:8443/conformancesuite/callback",
            "state":         "state_accounts",
            "nonce":         "5a6b0d7832a9fb4f80f1170a",
            "client_id":     client_id.clone().as_str(),
            "response_type": "code id_token",
        });
        let actual = super::PrivateClaims::new(client_id, consent_id);

        assert_eq!(serde_json::to_string(&actual).unwrap(), expected.to_string());
    }

    #[test]
    fn test_claims() {
        let consent_id: String = "stub_consent_id".into();
        let expected = serde_json::json!({
            "id_token": {
                "acr": {
                    "value": "urn:openbanking:psd2:sca",
                    "essential": true
                },
                "openbanking_intent_id": {
                    "value": consent_id.clone().as_str(),
                    "essential": true
                }
            },
            "userinfo": {
                "openbanking_intent_id": {
                    "value": consent_id.clone().as_str(),
                    "essential": true
                }
            }
        });
        let actual = super::Claims::new(consent_id.clone());

        assert_eq!(serde_json::to_string(&actual).unwrap(), expected.to_string());
    }

    #[test]
    #[ignore]
    fn test_jwt_new_good() {
        let client_id = "3fc528cf-fc88-46c2-9315-a8cf8724075d".to_string();
        let kid = "077825719abfc90f8b5645244a225510b834347a".to_string();
        let issuer = "https://modelobankauth2018.o3bank.co.uk:4101".to_string();
        let consent_id = "A02aff57e-80f9-4964-8548-4c9b17cfaa29".to_string();
        let request_object_signing_alg = "PS256".to_string();
        let jwt = super::JWT::new(client_id, kid, issuer, request_object_signing_alg, consent_id)
            .unwrap();

        let expected = "stub";
        let actual = jwt.0;

        assert_eq!(actual, expected);
    }
}
