extern crate reqwest;

use log::{debug, info};
use std::io::Read;

// Ozone response
// response={"access_token":"c4ddc2ae-7163-4c30-a0a8-c6b4f464a0d1","token_type":"Bearer","expires_in":3600}

pub fn main(config: super::config::Config, openidconfiguration: super::oidcdiscovery::OpenIDConfiguration) -> Result<(), Box<std::error::Error>> {
    let path = std::env::current_dir()?;
    debug!("cwd={}", path.display());

    let mut identity_buf = Vec::new();
    // {
    //     let mut key_file = std::fs::File::open(
    //         "keys/tls_client_auth_ps256/transport/755f353a31e0bfbf4e6317c9b52cfa37aa2bbbab.key",
    //     )?;
    //     key_file.read_to_end(&mut identity_buf)?;
    //     let mut pem_file = std::fs::File::open(
    //         "keys/tls_client_auth_ps256/transport/755f353a31e0bfbf4e6317c9b52cfa37aa2bbbab.pem",
    //     )?;
    //     pem_file.read_to_end(&mut identity_buf)?;
    // }
    {
        identity_buf.append(&mut "-----BEGIN PRIVATE KEY-----\nMIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQC9qQATeFvyikxf\nfM/qj3b3GHadyyoBr1S2VvBmW5VEVhklvmzv6fjOpHa2pp1obnsV3pm+FKrOrmrB\n3xm9B5Uzk3vQhLDBcPm70wW6mO2aAtLeKgW5whtM0SaehtemlALd4NNkjzHcMyTH\nLkffNrZ7xkVGsfC7pug0VD5zt6Js5CmZICEPQbYJGbqwQra4m4ES9QpBdnkvqyWL\ndl3VPGxjGY7bH19iSiaXlD9Ir44qLrH8y8NbFU4TUt3WLhiGI9zMNKCtbE9swaHm\nJR0VPcmKRzemmIToTaHQeNDSvWNCy2UZJxHnO6Xj5ZCv1vwkSphEr244Tx2VbrNe\n7iYa0ffpAgMBAAECggEBAJDljYZSfMTGveRxwZLtOjE7Qlv5PEV/QxPvRcidjWNg\n9+NrIWUFdq3A3mVa04VWarjhkMm0lm5CuJMXNF3DbkyyD3TV+wg+nLzPUmAlt9ji\n8WED783kKuE8JOoWEKlWCP6kZjw8XhZiCGYXyJcTAMV9S63gAhALSvm3puLXV8Mi\nF92rwLarTMJDsvFDpPj9nztjnpHy/xU3U8FDr8PsBpip0LIrsPplPLEDhbyXAdoh\nClnp4wIbcpw4c+2lae6HznXs+qDi567Sq/EVAc19J2pnaKtGVnDNk3jBrHpalR48\njBYG8QdJTyjPNXPrBCz9Ju/0U6N7Xm79S5/IbMgt7WUCgYEA7AqVcjbA+RF76Cnz\nwvBOy8OufoOKWYbbf6SjyHRRlPhxrmESogk+n0gb/m/Umi0+7NNy0tXABtEYYdYm\nlSL8y48+iHE2lgNVEnfEp+IsDuHQNqUmwRB1HAGGgR0IUZwvJMBot4B/v716hSen\nZ/aJpnBVP7SrwwmAoAyMNsqKrDcCgYEAzbJvwZpa3+avaOq+fB2m72sQr7LEpC+T\ndWhPjc7ts7rYVzMLWSDp/ggeFSv3rjIWEZr57RRxLQ/J99m3YAbHRCI9It2eH2Aj\nG+KO+InLlHyxn5sNc1Qyl0XCqFPDcGO1jb0Eu4aPcd7ykUFZRj9Ix9YayyKIdHZY\nMlTXdUO3rN8CgYEAjnwhFI361R75xf61GALJ/fTEu7gZ5ssiax3uhE27BYsnzJTq\nk4ezFqGHzIKjOuJ/LqTBloXTYgoMhtqn9bTmFtcEb4av5cRjMT+9JX7BC8bCUopF\nJ5wQpXRCRmRJJkJ6rtu7GrJl60+syv5R7s8C7sa2nzsnyBdPXs3UQirnpOcCgYBC\nW2v+EuN6E5tGx2PH+nDeD7YEWDFjng199LalUroQHxinmMnyaNHTp568ycBPK80+\nxLYXq16PYOaQ1GV5zvX9nuYGF2RpTsREvKPOvfU0470Rdh2ytNjVQCX503vbB++L\n2Axsyo3/+CJcha5dIRFDlmhyW6OVno/5PLF9HOlmFQKBgQC0P1n8JzR3iHiwL3g0\nD0lwHKAgku+ap604r01QvN9o6eczUKXjDKUXsR9nRhC+P5MhfAXYlhUUstdVUnPn\nq/tMJYGTXkGj7ZV+9ON27bp8aKM1Ecno0toIuy/aDYZasd6Xg0D3St0OtVIa4A7f\n+r5cuaoyKri7RDMV+QHX2a5cVA==\n-----END PRIVATE KEY-----\n".to_owned().into_bytes());
        identity_buf.append(&mut "-----BEGIN CERTIFICATE-----\nMIIFODCCBCCgAwIBAgIEWcV+HzANBgkqhkiG9w0BAQsFADBTMQswCQYDVQQGEwJH\nQjEUMBIGA1UEChMLT3BlbkJhbmtpbmcxLjAsBgNVBAMTJU9wZW5CYW5raW5nIFBy\nZS1Qcm9kdWN0aW9uIElzc3VpbmcgQ0EwHhcNMTkwNDA5MTA0ODU2WhcNMjAwNTA5\nMTExODU2WjBhMQswCQYDVQQGEwJHQjEUMBIGA1UEChMLT3BlbkJhbmtpbmcxGzAZ\nBgNVBAsTEjAwMTU4MDAwMDEwNDFSYkFBSTEfMB0GA1UEAxMWUkVmWktvN3pOMkll\nRTBYMlJGR1RiNDCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAL2pABN4\nW/KKTF98z+qPdvcYdp3LKgGvVLZW8GZblURWGSW+bO/p+M6kdramnWhuexXemb4U\nqs6uasHfGb0HlTOTe9CEsMFw+bvTBbqY7ZoC0t4qBbnCG0zRJp6G16aUAt3g02SP\nMdwzJMcuR982tnvGRUax8Lum6DRUPnO3omzkKZkgIQ9BtgkZurBCtribgRL1CkF2\neS+rJYt2XdU8bGMZjtsfX2JKJpeUP0ivjiousfzLw1sVThNS3dYuGIYj3Mw0oK1s\nT2zBoeYlHRU9yYpHN6aYhOhNodB40NK9Y0LLZRknEec7pePlkK/W/CRKmESvbjhP\nHZVus17uJhrR9+kCAwEAAaOCAgQwggIAMA4GA1UdDwEB/wQEAwIHgDAgBgNVHSUB\nAf8EFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwgeAGA1UdIASB2DCB1TCB0gYLKwYB\nBAGodYEGAWQwgcIwKgYIKwYBBQUHAgEWHmh0dHA6Ly9vYi50cnVzdGlzLmNvbS9w\nb2xpY2llczCBkwYIKwYBBQUHAgIwgYYMgYNVc2Ugb2YgdGhpcyBDZXJ0aWZpY2F0\nZSBjb25zdGl0dXRlcyBhY2NlcHRhbmNlIG9mIHRoZSBPcGVuQmFua2luZyBSb290\nIENBIENlcnRpZmljYXRpb24gUG9saWNpZXMgYW5kIENlcnRpZmljYXRlIFByYWN0\naWNlIFN0YXRlbWVudDBtBggrBgEFBQcBAQRhMF8wJgYIKwYBBQUHMAGGGmh0dHA6\nLy9vYi50cnVzdGlzLmNvbS9vY3NwMDUGCCsGAQUFBzAChilodHRwOi8vb2IudHJ1\nc3Rpcy5jb20vb2JfcHBfaXNzdWluZ2NhLmNydDA6BgNVHR8EMzAxMC+gLaArhilo\ndHRwOi8vb2IudHJ1c3Rpcy5jb20vb2JfcHBfaXNzdWluZ2NhLmNybDAfBgNVHSME\nGDAWgBRQc5HGIXLTd/T+ABIGgVx5eW4/UDAdBgNVHQ4EFgQUanhMVcNxUI03lzht\nM0Ap9Uqe9MYwDQYJKoZIhvcNAQELBQADggEBAA+Pxffl5XELhA5X2k7eL4nqqnR8\n2DWn5iG6sHfdJOUwUlsIewyTB7M6seYiSu8ezrWfyVASqYJUqQacNVc1Q0DncmqU\nRBetAsGNWh1hBVB7mTci54CGnqc3WAZZ9Mkl326uceNVEcE5HQ/wbynDqaZzJb7k\nqJlfaSZgSptV22dYnSX8ZWG7AWFYWWXytCUw29KLUZv4QDtSpOUZOP98GWkDXgEo\n082GaJjr4IS7BlNUVtICQGVFZ9RvJr7yAiscQTSKII+viHa+8jtaGweHKr69oAaI\nzvMQ1hK9jFaNRaYSK6eNgEncQSddd9U04x65N+uyHUd1qG39gtEipxOVlMs=\n-----END CERTIFICATE-----\n".to_owned().into_bytes());
    }
    let pkcs12 = reqwest::Identity::from_pem(&identity_buf)?;
    debug!("pkcs12={:?}", pkcs12);

    let cert_issuing_ca = reqwest::Certificate::from_pem(CERT_ISSUING_CA.as_bytes())?;
    debug!("cert_issuing_ca={:?}", cert_issuing_ca);
    let cert_root_ca = reqwest::Certificate::from_pem(CERT_ROOT_CA.as_bytes())?;
    debug!("cert_root_ca={:?}", cert_root_ca);

    let mut headers = reqwest::header::HeaderMap::new();
    {
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "banaio-openbankingforgerock/0.1.0 (https://github.com/banaio/openbanking.rs)",
            ),
        );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("no-cache"),
        );
        debug!("headers={:?}", headers);
    }

    let client = reqwest::Client::builder()
        .add_root_certificate(cert_issuing_ca)
        .add_root_certificate(cert_root_ca)
        .identity(pkcs12)
        .use_rustls_tls()
        .default_headers(headers)
        .build()?;
    let mut params = std::collections::HashMap::new();
    params.insert("grant_type", "client_credentials");
    // params.insert("scope", "openid payments accounts fundsconfirmations");
    // params.insert("scope", "fundsconfirmations");
    params.insert("scope", "payments accounts");
    // params.insert("client_id", config.client_id.as_str());
    params.insert("client_id", "3fc528cf-fc88-46c2-9315-a8cf8724075d");
    debug!("params={:?}", params);

    // "https://matls.as.aspsp.ob.forgerock.financial/oauth2/access_token"
    let url = openidconfiguration.token_endpoint.as_str();
    let request = client
        .post(url)
        .form(&params);
    info!("url={}", url);
    info!("request={:?}", request);
    debug!("request={:?}", request);

    let mut response = request.send().expect("request.send() failed");
    // std::io::copy(&mut response, &mut std::io::stdout())?;

    let mut response_buf = String::new();
    response.read_to_string(&mut response_buf).expect("Failed to read response");
    info!("response={}", response_buf);

    Ok(())
}

const CERT_ISSUING_CA: &str = "-----BEGIN CERTIFICATE-----
MIIGEzCCA/ugAwIBAgIEWcT9RzANBgkqhkiG9w0BAQsFADBQMQswCQYDVQQGEwJH
QjEUMBIGA1UEChMLT3BlbkJhbmtpbmcxKzApBgNVBAMTIk9wZW5CYW5raW5nIFBy
ZS1Qcm9kdWN0aW9uIFJvb3QgQ0EwHhcNMTcwOTIyMTI0NjU3WhcNMjcwOTIyMTMx
NjU3WjBTMQswCQYDVQQGEwJHQjEUMBIGA1UEChMLT3BlbkJhbmtpbmcxLjAsBgNV
BAMTJU9wZW5CYW5raW5nIFByZS1Qcm9kdWN0aW9uIElzc3VpbmcgQ0EwggEiMA0G
CSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCyyrRg2jF01jXhX3IR44p338ZBozn8
WkZaCN8MB+AlBfuXHD6mC/0v+N/Z4XI6E5pzArmTho8D6a6JDpAHmmefqGSqOXVb
clYv1tHFjmC1FtKqkFHTTMyhl41nEMo0dnvWA45bMsGm0yMi/tEM5Vb5dSY4Zr/2
LWgUTDFUisgUbyIIHT+L6qxPUPCpNuEd+AWVc9K0SlmhaC+UIfVO83gE1+9ar2dO
NSFaK/a445Us6MnqgKvfkvKdaR06Ok/EhGgiAZORcyZ61EYFVVzJewy5NrFSF3mw
iPYvMxoT5bxcwAEvxqBXpTDv8njQfR+cgZDeloeK1UqmW/DpR+jj3KNHAgMBAAGj
ggHwMIIB7DAOBgNVHQ8BAf8EBAMCAQYwEgYDVR0TAQH/BAgwBgEB/wIBADCB4AYD
VR0gBIHYMIHVMIHSBgsrBgEEAah1gQYBZDCBwjAqBggrBgEFBQcCARYeaHR0cDov
L29iLnRydXN0aXMuY29tL3BvbGljaWVzMIGTBggrBgEFBQcCAjCBhgyBg1VzZSBv
ZiB0aGlzIENlcnRpZmljYXRlIGNvbnN0aXR1dGVzIGFjY2VwdGFuY2Ugb2YgdGhl
IE9wZW5CYW5raW5nIFJvb3QgQ0EgQ2VydGlmaWNhdGlvbiBQb2xpY2llcyBhbmQg
Q2VydGlmaWNhdGUgUHJhY3RpY2UgU3RhdGVtZW50MGoGCCsGAQUFBwEBBF4wXDAy
BggrBgEFBQcwAoYmaHR0cDovL29iLnRydXN0aXMuY29tL29idGVzdHJvb3RjYS5j
cnQwJgYIKwYBBQUHMAGGGmh0dHA6Ly9vYi50cnVzdGlzLmNvbS9vY3NwMDcGA1Ud
HwQwMC4wLKAqoCiGJmh0dHA6Ly9vYi50cnVzdGlzLmNvbS9vYl9wcF9yb290Y2Eu
Y3JsMB8GA1UdIwQYMBaAFOw4jgva8/k3PpDefV9q5mDNeUKDMB0GA1UdDgQWBBRQ
c5HGIXLTd/T+ABIGgVx5eW4/UDANBgkqhkiG9w0BAQsFAAOCAgEAdRg2H9uLwzlG
qvHGjIz0ydM1tElujEcWJp5MeiorikK0rMOlxVU6ZFBlXPfO1APu0cZXxfHwWs91
zoNCpGXebC6tiDFQ3+mI4qywtippjBqb6Sft37NlkXDzQETomsY7wETuUJ31xFA0
FccI8WlAUzUOBE8OAGo5kAZ4FTa/nkd8c2wmuwSp+9/s+gQe0K9BkxywoP1WAEdU
AaKW3RE9yuTbHA/ZF/zz4/Rpw/FB/hYhOxvDV6qInl5B7ErSH4r4v4D2jiE6apAc
n5LT+e0aBa/EgGAxgyAgrYpw1s+TCUJot+227xRvXxeeZzXa2igsd+C845BGiSlt
hzr0mqYDYEWJMfApZ+BlMtxa7K9T3D2l6XMv12RoNnEWe6H5xazTvBLiTibW3c5i
j8WWKJNtQbgmooRPaKJIl+0rm54MFH0FDxJ+P4mAR6qa8JS911nS26iCsE9FQVK5
1djuct349FYBOVM595/GkkTz9k1vXw1BdD71lNjI00Yjf73AAtvL/X4CpRz92Nag
shS2Ia5a3qjjFrjx7z4h7QtMJGjuUsjTI/c+yjIYwAZ5gelF5gz7l2dn3g6B40pu
7y1EewlfIQh/HVMF0ZpF29XL6+7siYQCGhP5cNJ04fotzqDPaT2XlOhE3yNkjp82
uzCWvhLUJgE3D9V9PL0XD/ykNEP0Fio=
-----END CERTIFICATE-----

";

const CERT_ROOT_CA: &str = "-----BEGIN CERTIFICATE-----
MIIFYDCCA0igAwIBAgIEWcT89jANBgkqhkiG9w0BAQsFADBQMQswCQYDVQQGEwJH
QjEUMBIGA1UEChMLT3BlbkJhbmtpbmcxKzApBgNVBAMTIk9wZW5CYW5raW5nIFBy
ZS1Qcm9kdWN0aW9uIFJvb3QgQ0EwHhcNMTcwOTIyMTEzOTQyWhcNMzcwOTIyMTIw
OTQyWjBQMQswCQYDVQQGEwJHQjEUMBIGA1UEChMLT3BlbkJhbmtpbmcxKzApBgNV
BAMTIk9wZW5CYW5raW5nIFByZS1Qcm9kdWN0aW9uIFJvb3QgQ0EwggIiMA0GCSqG
SIb3DQEBAQUAA4ICDwAwggIKAoICAQCYuk3s8HqlNQL6ahJuHybswCgIWkDv6U7O
3WbNWNeIR3PdAEd23EppcT2r6z96+TrLqGCgdf6z2YouLOt62A2JqJ4iJaVFjt0e
9j5bQqPhNEVieSqNB8xhUlpGFEXdj7GPw03qvSeqGITSfjsV9Vi8NZgUqtYKdZPV
a1FL3vR1YVN052nO/tFewoFn5AdEC/SrpIPyXk50SvXzbx9vBJnA6MLJ8CoI9yNy
I7j6QyL35OeUf6c7fDTkLB2Vf29RjL/YhJy70GXt0sUbL5N9Rezr8JlhwGEPRpr0
D+rKyYLoGWLUtoSvYwPC85ePMWmdlUwOaC59NLUihiy4uszE2qP2CJslHdOhgWkC
Q86K+yga+lCh6GX7qPZKNnS3YsjZ+23o1+ZNzbKcOGsbCdP/hXzTa/D8fyo/0MTM
qXsEbOqqocD1CfGNv1Bfn2rbe41oKAv2fiEYf4Qrq9f16jrTK4f3bUA1V7xiu2YR
DFdSJ7CZvtrpLwPXP/WeYZaeO+SJQq2+Ag67aEi+9A+Zy+QjWwjCoqgNJGRZiEjQ
hU7gX6Nz9GeRNV3RGHUKUqRtFrMxvG0M1FjODC7kosEscZtI4FkBTLtAoV4XqOsL
tfm2kxrTA58zf+dnlIb51Mne63f5GUd4+Cyb3SUcvqFj4FQtzIO44FvArraFkCvC
/vi5IYANvQIDAQABo0IwQDAOBgNVHQ8BAf8EBAMCAQYwDwYDVR0TAQH/BAUwAwEB
/zAdBgNVHQ4EFgQU7DiOC9rz+Tc+kN59X2rmYM15QoMwDQYJKoZIhvcNAQELBQAD
ggIBABidG6pXhNLmLOzHl7lP3j/SEE4Mq9DQLkirfM3JeXmcOB0xKwqvpzJSjeKx
9+hYlQRKorIc5QD1hLKqjG7CqUNcCgPGXU96iofcUPthFI+fgkJL6yMOtlLiidlF
lchYmIHk2SjhMUP/TQzwHe2Ca0ssCRgyJ1DEkaxkurEb2H8jZVpEVlHJrc3itxzY
m4Un11Zh+brNvrZfc2Phy4fu3HckphxCsJ76ZbpyiuZ7lWnDT6hONH+zEHITsEmY
sje1xRiDR3MJSty7xf06qe7UEKtsFgwGuzS/4IQIRKKfgk1RUZH6WxK+ZsB9HsYV
P/sSVEZWNUT3z9N4hveIrP/So0A8toAID0nNOqC5o9yBdQTKN9IX5Neh801PTX18
3/t/OC2JEFAPvo6IxXtMpfI2aTxrpBWKIBX5xgFs5lJH7nJ8CKkWIdGo6hhrx8If
lpAKuq+YXLhWFWuyyP6R8VfRXWRv3ZiZVXrgsDx2RA4Q2r4AE2BjGxyYW/vxF73B
3Yf0yi1IBOQ9d+VqT8hSO76zYWY+1r5+qWzsqT4tIIWG6ZA8ckcnpjnxgM/XnDwt
JICq3DR8C2rSeyed3x9CCtAbgKpvc2bGWZtISZti93Smy5aotMm+4aKpm7EZRfSs
D2knnIn31bnaW3SDokLiG7OnFFU2lMkcrlFFsTeWq5Sn/XUD
-----END CERTIFICATE-----

";
