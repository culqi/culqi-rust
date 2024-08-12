use reqwest::header::{HeaderMap, HeaderName};
use serde_json::Value;

pub struct CustomHeaders;

impl CustomHeaders {
    pub fn get_headers(key: &str) -> HeaderMap {
        const X_CULQI_ENV_TEST: &str = "test";
        const X_CULQI_ENV_LIVE: &str = "live";
        const X_CULQI_CLIENT: &str = "culqi-rust";
        const X_CULQI_CLIENT_VERSION: &str = "1.0.1";
        const X_API_VERSION: &str = "2";

        let mut env = X_CULQI_ENV_LIVE;

        if key.contains("test") {
            env = X_CULQI_ENV_TEST;
        }

        let mut headers = HeaderMap::new();
        headers.insert("x-culqi-env", env.parse().unwrap());
        headers.insert("x-api-version", X_API_VERSION.parse().unwrap());
        headers.insert("x-culqi-client", X_CULQI_CLIENT.parse().unwrap());
        headers.insert(
            "x-culqi-client-version",
            X_CULQI_CLIENT_VERSION.parse().unwrap(),
        );

        return headers;
    }

    pub fn get_custom_headers(key: &str, custom_headers: &str) -> HeaderMap {
        const X_CULQI_ENV_TEST: &str = "test";
        const X_CULQI_ENV_LIVE: &str = "live";
        const X_CULQI_CLIENT: &str = "culqi-rust";
        const X_CULQI_CLIENT_VERSION: &str = "1.0.1";
        const X_API_VERSION: &str = "2";

        let mut env = X_CULQI_ENV_LIVE;

        if key.contains("test") {
            env = X_CULQI_ENV_TEST;
        }

        let mut headers = HeaderMap::new();
        headers.insert("x-culqi-env", env.parse().unwrap());
        headers.insert("x-api-version", X_API_VERSION.parse().unwrap());
        headers.insert("x-culqi-client", X_CULQI_CLIENT.parse().unwrap());
        headers.insert(
            "x-culqi-client-version",
            X_CULQI_CLIENT_VERSION.parse().unwrap(),
        );

        if !custom_headers.is_empty() {
            if let Ok(custom_headers_value) = serde_json::from_str::<Value>(custom_headers) {
                println!("Headrers Add: {:?}", custom_headers_value);

                for (header_key, header_value) in custom_headers_value.as_object().unwrap().iter() {
                    if !header_value.is_null() {
                        let value_str = header_value.to_string();
                        headers.insert(
                            HeaderName::from_bytes(header_key.as_bytes()).unwrap(),
                            value_str.parse().unwrap(),
                        );
                    }
                }
            }
        }

        return headers;
    }
}