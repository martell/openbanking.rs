use biscuit;
use chrono;
use serde;
use uuid;

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
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Claims {
    id_token: IdToken,
    userinfo: UserInfo,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct IdToken {
    acr: Acr,
    openbanking_intent_id: OpenbankingIntentId,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserInfo {
    openbanking_intent_id: OpenbankingIntentId,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Acr {
    value: String,
    essential: bool,
}

impl Default for Acr {
    fn default() -> Acr {
        Acr {
            value: "urn:openbanking:psd2:sca".into(),
            essential: true,
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OpenbankingIntentId {
    value: String,
    essential: bool,
}

impl Default for OpenbankingIntentId {
    fn default() -> OpenbankingIntentId {
        let default = OpenbankingIntentId {
            essential: true,
            value: "".into(),
        };
        default
    }
}

impl Claims {
    fn new(consent_id: String) -> Claims {
        let claims = Claims {
            id_token: IdToken {
                acr: Acr {
                    ..Default::default()
                },
                openbanking_intent_id: OpenbankingIntentId {
                    value: consent_id.clone(),
                    essential: true,
                },
            },
            userinfo: UserInfo {
                openbanking_intent_id: OpenbankingIntentId {
                    value: consent_id.clone(),
                    essential: true,
                },
            },
        };

        claims
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
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateClaims {
    scope: String,
    claims: Claims,
    redirect_uri: String,
    state: String,
    nonce: String,
    client_id: String,
    response_type: String,
}

pub struct JWT;

impl JWT {
    // forgerock `request`:
    // eyJhbGciOiJSUzI1NiIsImtpZCI6ImQ2YzNmNDlkLTcxMTItNGM1Yy05YzlkLTg0OTI2ZTk5MmM3NCIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJodHRwczovL21hdGxzLmFzLmFzcHNwLm9iLmZvcmdlcm9jay5maW5hbmNpYWwvb2F1dGgyL29wZW5iYW5raW5nIiwiY2xhaW1zIjp7ImlkX3Rva2VuIjp7ImFjciI6eyJlc3NlbnRpYWwiOnRydWUsInZhbHVlIjoidXJuOm9wZW5iYW5raW5nOnBzZDI6c2NhIn0sIm9wZW5iYW5raW5nX2ludGVudF9pZCI6eyJlc3NlbnRpYWwiOnRydWUsInZhbHVlIjoiQWJjM2UwOGJjLTcyYzUtNGUzMy1hYjYwLThiZDlhZjhhZGMxNiJ9fSwidXNlcmluZm8iOnsib3BlbmJhbmtpbmdfaW50ZW50X2lkIjp7ImVzc2VudGlhbCI6dHJ1ZSwidmFsdWUiOiJBYmMzZTA4YmMtNzJjNS00ZTMzLWFiNjAtOGJkOWFmOGFkYzE2In19fSwiY2xpZW50X2lkIjoiNTRmNjQzMDktNDMzZC00NjEwLTk1ZDItNjNkMmY1MjUzNDEyIiwiZXhwIjoxNTQwMTk3OTk5LCJpYXQiOjE1NDAxOTc4NzksImlzcyI6IjU0ZjY0MzA5LTQzM2QtNDYxMC05NWQyLTYzZDJmNTI1MzQxMiIsImp0aSI6IjJmODMyMzJjLTA0NmUtNGIyMC05NTc4LWRmMTljOTdhZTNmOSIsIm5vbmNlIjoiNWE2YjBkNzgzMmE5ZmI0ZjgwZjExNzBhIiwicmVkaXJlY3RfdXJpIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwL29wZW5iYW5raW5nL2JhbmFpby9mb3JnZXJvY2siLCJyZXNwb25zZV90eXBlIjoiY29kZSBpZF90b2tlbiIsInNjb3BlIjoiYWNjb3VudHMgb3BlbmlkIiwic3RhdGUiOiI1YTZiMGQ3ODMyYTlmYjRmODBmMTE3MGEifQ.KuTvvOC2yz5idjUVH6I7aZlHj0jGuR06zJlNny8D01XoHvm0xA27YXyIwsQO-q0MlMDErBzzU8WMZ3-6wJxWth4thPmSu5zzVAYo7ZWEUDHhlq7YWZkATRintLv0PqUlx_h8r8N2tmtm0UWE2VtxKdRQN1jSD7_kjsw7w_vaP_OwvOA8lGEjU30JW4HxHLfxyeIjHxsTY_dlSiHvWwdmqlwEW9DQJtAYHGboJkX6GBXqV5zEHD4UdtjRYIkyPDAgHqt5smiEzMcuGwJoD2v4vSBEmpEdnmAANgPFxKhNsyNhm7HQXaL6vRLuasgrg7JW9F8iWvw-4BlASAcoBiwKCg
    pub fn new(
        client_id: String,
        kid: String,
        issuer: String,
        request_object_signing_alg: String,
        consent_id: String,
    ) -> Result<String, Box<std::error::Error>> {
        use biscuit::SingleOrMultiple;
        use biscuit::StringOrUri;

        let expiry = Some(std::convert::From::from(
            chrono::Utc::now() + chrono::Duration::minutes(29),
        ));
        let issued_at = Some(std::convert::From::from(chrono::Utc::now()));
        let id = Some(uuid::Uuid::new_v4().to_hyphenated().to_string());
        let claims = Claims::new(consent_id);
        let private = PrivateClaims {
            scope: "openid accounts".into(),
            claims: claims,
            redirect_uri: "https://127.0.0.1:8443/conformancesuite/callback".into(),
            state: "state_accounts".into(),
            nonce: "5a6b0d7832a9fb4f80f1170a".into(),
            client_id: client_id.clone(),
            response_type: "code id_token".into(),
        };
        let registered = biscuit::RegisteredClaims {
            issuer: Some(StringOrUri::String(client_id.clone())),
            subject: Some(StringOrUri::String(client_id.clone())),
            audience: Some(SingleOrMultiple::Single(StringOrUri::String(
                issuer.clone(),
            ))),
            expiry: expiry,
            issued_at: issued_at,
            id: id,
            ..Default::default()
        };
        let payload = biscuit::ClaimsSet::<PrivateClaims> {
            registered: registered,
            private: private,
        };

        let algorithm = match request_object_signing_alg.as_str() {
            "PS256" => biscuit::jwa::SignatureAlgorithm::PS256,
            "RS256" => biscuit::jwa::SignatureAlgorithm::RS256,
            _ => biscuit::jwa::SignatureAlgorithm::PS256,
        };
        let jwt = biscuit::JWT::new_decoded(
            From::from(biscuit::jws::RegisteredHeader {
                algorithm: algorithm,
                key_id: Some(kid.clone()),
                ..Default::default()
            }),
            payload.clone(),
        );

        // let signing_secret =
        //     biscuit::jws::Secret::rsa_keypair_from_file("src/client/private_key.der").unwrap();
        let signing_secret = biscuit::jws::Secret::rsa_keypair_from_file(
            "keys/config_tls_client_auth_ps256_ozone/private_key.der",
        )?;
        // let signing_secret = biscuit::jws::Secret::Bytes("secret".to_string().into_bytes());
        let token = jwt.into_encoded(&signing_secret)?;
        let token = token.unwrap_encoded().to_string();

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_jwt_new() {
        let client_id = "3fc528cf-fc88-46c2-9315-a8cf8724075d".to_string();
        let kid = "077825719abfc90f8b5645244a225510b834347a".to_string();
        let issuer = "https://modelobankauth2018.o3bank.co.uk:4101".to_string();
        let consent_id = "A02aff57e-80f9-4964-8548-4c9b17cfaa29".to_string();
        let request_object_signing_alg = "PS256".to_string();
        let jwt = super::JWT::new(
            client_id,
            kid,
            issuer,
            request_object_signing_alg,
            consent_id,
        );

        let expected = "stub";
        let actual = jwt.unwrap();
        println!("expected={:?}", expected);
        println!("actual={:?}", actual);

        assert_eq!(actual, expected);
    }
}
