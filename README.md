# `openbanking.rs`

Connect to either:

* [Integrating a TPP with Ozone Model Banks Using Postman](https://openbanking.atlassian.net/wiki/spaces/DZ/pages/32374785/Integrating+a+TPP+with+Ozone+Model+Banks+Using+Postman+-+Draft).
* [Integrating a TPP with ForgeRock Model Bank on Directory Sandbox](https://openbanking.atlassian.net/wiki/spaces/DZ/pages/187793608/Integrating+a+TPP+with+ForgeRock+Model+Bank+on+Directory+Sandbox).

More information [Do you have test environments for TPPs including a sandbox?](https://openbanking.atlassian.net/wiki/spaces/DZ/pages/22872552/Do+you+have+test+environments+for+TPPs+including+a+sandbox).

## Builds [![CircleCI](https://circleci.com/gh/banaio/openbanking.rs.svg?style=svg)](https://circleci.com/gh/banaio/openbanking.rs)

[`https://circleci.com/gh/banaio/openbanking.rs`](https://circleci.com/gh/banaio/openbanking.rs)

## Example

```sh
$ cargo test
   Compiling openbanking v0.1.0 (/Users/mbana/dev/banaio/github/openbanking.rs)
    Finished dev [unoptimized + debuginfo] target(s) in 8.30s
     Running target/debug/deps/openbanking-0dfe4bba1bc6ed58

running 9 tests
test client::claims::tests::test_jwt_new_good ... ignored
test client::client_credentials::tests::test_deserialize_good ... ok
test client::accounts::tests::test_to_string_good ... ok
test config::tests::test_read_config_bad ... ok
test client::claims::tests::test_claims ... ok
test client::claims::tests::test_private_claims ... ok
test http::tests::test_default_headers ... ok
test oidcdiscovery::tests::test_deserialize_good ... ok
test config::tests::test_read_config_good ... ok

test result: ok. 8 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

$ cargo run
   Compiling openbanking v0.1.0 (/Users/mbana/dev/banaio/github/openbanking)
    Finished dev [unoptimized + debuginfo] target(s) in 4.08s
     Running `target/debug/openbanking`
 INFO 2019-06-10T12:32:28Z: openbanking::oidcdiscovery: openidconfiguration=OpenIDConfiguration { request_parameter_supported: true, claims_parameter_supported: true, request_uri_parameter_supported: true, introspection_endpoint: "https://matls.as.aspsp.ob.forgerock.financial/oauth2/introspect", issuer: "https://as.aspsp.ob.forgerock.financial/oauth2", authorization_endpoint: "https://as.aspsp.ob.forgerock.financial/oauth2/authorize", token_endpoint: "https://matls.as.aspsp.ob.forgerock.financial/oauth2/access_token", version: "3.1", userinfo_endpoint: "https://matls.as.aspsp.ob.forgerock.financial/oauth2/userinfo", jwks_uri: "https://as.aspsp.ob.forgerock.financial/api/jwk/jwk_uri", registration_endpoint: "https://matls.as.aspsp.ob.forgerock.financial/open-banking/register/", require_request_uri_registration: true, grant_types_supported: ["refresh_token", "client_credentials", "authorization_code"], scopes_supported: ["openid", "payments", "fundsconfirmations", "accounts"], id_token_encryption_enc_values_supported: ["A256GCM", "A192GCM", "A128GCM", "A128CBC-HS256", "A192CBC-HS384", "A256CBC-HS512"], acr_values_supported: ["urn:openbanking:psd2:sca", "urn:openbanking:psd2:ca"], request_object_encryption_enc_values_supported: ["A256GCM", "A192GCM", "A128GCM", "A128CBC-HS256", "A192CBC-HS384", "A256CBC-HS512"], claims_supported: ["acr", "zoneinfo", "openbanking_intent_id", "address", "profile", "name", "phone_number", "given_name", "locale", "family_name", "email"], token_endpoint_auth_methods_supported: ["client_secret_post", "private_key_jwt", "client_secret_basic", "tls_client_auth"], response_types_supported: ["code token id_token", "code", "code id_token", "device_code", "id_token", "code token", "token", "token id_token"], id_token_encryption_alg_values_supported: ["RSA-OAEP", "RSA-OAEP-256", "A128KW", "A256KW", "RSA1_5", "dir", "A192KW"], subject_types_supported: ["public", "pairwise"], id_token_signing_alg_values_supported: ["RS256", "PS256"], request_object_signing_alg_values_supported: ["RS256", "PS256"], request_object_encryption_alg_values_supported: ["RSA-OAEP", "RSA-OAEP-256", "A128KW", "RSA1_5", "A256KW", "dir", "A192KW"], userinfo_signing_alg_values_supported: ["ES384", "HS256", "HS512", "ES256", "RS256", "HS384", "ES512"], userinfo_encryption_enc_values_supported: ["A256GCM", "A192GCM", "A128GCM", "A128CBC-HS256", "A192CBC-HS384", "A256CBC-HS512"], userinfo_encryption_alg_values_supported: ["RSA-OAEP", "RSA-OAEP-256", "A128KW", "A256KW", "RSA1_5", "dir", "A192KW"], token_endpoint_auth_signing_alg_values_supported: ["RS256", "PS256"] }
 INFO 2019-06-10T12:32:29Z: openbanking::client: response={"access_token":"eyJ0eXAiOiJKV1QiLCJ6aXAiOiJOT05FIiwia2lkIjoiRTE5N1kzMVFLT05mSk42aTdrQlkyMzFneUFvPSIsImFsZyI6IkVTMjU2In0.eyJzdWIiOiI0YzYyNTU1NS0yZTI5LTQxNGEtYjZlMC1mOGNiNzgyNTZmZGYiLCJjdHMiOiJPQVVUSDJfU1RBVEVMRVNTX0dSQU5UIiwiYXVkaXRUcmFja2luZ0lkIjoiOGEwNTFlODAtYzY4ZS00MDhhLWIxN2ItMWEzM2FhNGE0NmUxLTEwMTQwODIiLCJpc3MiOiJodHRwczovL2FzLmFzcHNwLm9iLmZvcmdlcm9jay5maW5hbmNpYWwvb2F1dGgyIiwidG9rZW5OYW1lIjoiYWNjZXNzX3Rva2VuIiwidG9rZW5fdHlwZSI6IkJlYXJlciIsImF1dGhHcmFudElkIjoiN0otUkF4WjVqNFg1ZU5yTmRNTG5NNGxzS0dJIiwiYXVkIjoiNGM2MjU1NTUtMmUyOS00MTRhLWI2ZTAtZjhjYjc4MjU2ZmRmIiwibmJmIjoxNTYwMTY5OTQ4LCJncmFudF90eXBlIjoiY2xpZW50X2NyZWRlbnRpYWxzIiwic2NvcGUiOlsib3BlbmlkIiwicGF5bWVudHMiLCJmdW5kc2NvbmZpcm1hdGlvbnMiLCJhY2NvdW50cyJdLCJhdXRoX3RpbWUiOjE1NjAxNjk5NDgsInJlYWxtIjoiL29wZW5iYW5raW5nIiwiZXhwIjoxNTYwMjU2MzQ4LCJpYXQiOjE1NjAxNjk5NDgsImV4cGlyZXNfaW4iOjg2NDAwLCJqdGkiOiJmS3hBS09rRU83aUU3Z2xOVTlkUTQydG13VncifQ.gkMU9OaTvlkni0kYhTzp5JObYA2BDimkv-upfIfcyhXoQcDcDV9idS8RILXK_5Ud8k6ibNSkSptQrmT7YT831g","expires_in":86399,"id_token":"eyJraWQiOiIyYzk3ZDdmOWQyYjRkNTE5OTI4MDM2MGVkZTMzZTYzZDQ4MTUwOTRkIiwiYWxnIjoiUFMyNTYifQ.eyJhdF9oYXNoIjoiMkZJWDFLZHZEMGdhVTZxWlA2MEFtdyIsInN1YiI6IjRjNjI1NTU1LTJlMjktNDE0YS1iNmUwLWY4Y2I3ODI1NmZkZiIsImF1ZGl0VHJhY2tpbmdJZCI6IjhhMDUxZTgwLWM2OGUtNDA4YS1iMTdiLTFhMzNhYTRhNDZlMS0xMDE0MDgzIiwiaXNzIjoiaHR0cHM6XC9cL2FzLmFzcHNwLm9iLmZvcmdlcm9jay5maW5hbmNpYWxcL29hdXRoMiIsInRva2VuTmFtZSI6ImlkX3Rva2VuIiwiYXVkIjoiNGM2MjU1NTUtMmUyOS00MTRhLWI2ZTAtZjhjYjc4MjU2ZmRmIiwiYXpwIjoiNGM2MjU1NTUtMmUyOS00MTRhLWI2ZTAtZjhjYjc4MjU2ZmRmIiwiYXV0aF90aW1lIjoxNTYwMTY5OTQ4LCJyZWFsbSI6Ilwvb3BlbmJhbmtpbmciLCJleHAiOjE1NjAyNTYzNDgsInRva2VuVHlwZSI6IkpXVFRva2VuIiwiaWF0IjoxNTYwMTY5OTQ5LCJqdGkiOiJkOTRmZjk0NC1kM2Q2LTRkMDEtOWEwYy1hMzQyMGQ5ZjU2OTAifQ.DykB-XTAZdB-YxzlLoHS7BZM8HlhOhBM5Orrrw3Pazprh3WmXKGHICIfqn2mHdvFOvTb9rdI51w8av195zkrTt3c0KJjEf0URErXdyJ-zldKlk-VdSJKiuTFFvLd6842vMUNlA52Haxc1PgYutng9Cd0ivEtHFG8n0j-aQ7TMiZvAGeTJregTnw-Cc40gtfMD5Bod50_-PmxqEJFBLoM0xFE7GcgSrLH-hzCJNV1m2mSlSTBNmusRp7MIsFrjwg9E7n7lqaCxp54ubqiyp-ViuD-QsFZuVB9MH7LaCZo-NDSGGCJaYj1qHsVRyC3j3khMQDyq2N8AN6zo8RH6Wrcmw","token_type":"Bearer","scope":"openid payments fundsconfirmations accounts"}
```

## Swagger

The Swagger files are available at <https://github.com/OpenBankingUK/read-write-api-specs/tree/master/dist>.

### Account and Transaction API Specification - v3.1

* JSON: <https://raw.githubusercontent.com/OpenBankingUK/read-write-api-specs/v3.1.0/dist/account-info-swagger.json>.
* YAML: <https://raw.githubusercontent.com/OpenBankingUK/read-write-api-specs/v3.1.0/dist/account-info-swagger.yaml>.

### Payment Initiation API Specification - v3.1

* JSON: <https://raw.githubusercontent.com/OpenBankingUK/read-write-api-specs/v3.1.0/dist/payment-initiation-swagger.json>.
* YAML: <https://raw.githubusercontent.com/OpenBankingUK/read-write-api-specs/v3.1.0/dist/payment-initiation-swagger.yaml>.
