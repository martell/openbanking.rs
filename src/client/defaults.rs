use reqwest;
use log::debug;

pub fn headers() -> reqwest::header::HeaderMap {
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
    use super::*;

    #[test]
    fn test_headers() {
        let mut expected = reqwest::header::HeaderMap::new();
        expected.insert("user-agent", "reqwest/0.9.18 banaio-openbankingforgerock/0.1.0 (https://github.com/banaio/openbanking.rs)".parse().unwrap());
        expected.insert("cache-control", "no-cache".parse().unwrap());

        let actual = headers();
        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }
}
