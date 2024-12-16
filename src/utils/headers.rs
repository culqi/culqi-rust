use reqwest::header::{HeaderMap, HeaderName};
use serde_json::Value;

pub struct CustomHeaders;

impl CustomHeaders {

    const X_CULQI_ENV_TEST: &'static str = "test";
    const X_CULQI_ENV_LIVE: &'static str = "live";
    const X_CULQI_CLIENT: &'static str = "culqi-rust";
    const X_CULQI_CLIENT_VERSION: &'static str = "1.0.1";
    const X_API_VERSION: &'static str = "2";

    fn create_base_headers(env: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("x-culqi-env", env.parse().expect("Failed to parse x-culqi-env"));
        headers.insert("x-api-version", Self::X_API_VERSION.parse().expect("Failed to parse x-api-version"));
        headers.insert("x-culqi-client", Self::X_CULQI_CLIENT.parse().expect("Failed to parse x-culqi-client"));
        headers.insert("x-culqi-client-version", Self::X_CULQI_CLIENT_VERSION.parse().expect("Failed to parse x-culqi-client-version"));
        return headers
    }

    pub fn get_headers(key: &str) -> HeaderMap {
        let env = if key.contains("test") {
            Self::X_CULQI_ENV_TEST
        } else {
            Self::X_CULQI_ENV_LIVE
        };

        let headers =  Self::create_base_headers(env);

        return headers;
    }

    pub fn get_custom_headers(key: &str, custom_headers: &str) -> HeaderMap {
        let env = if key.contains("test") {
            Self::X_CULQI_ENV_TEST
        } else {
            Self::X_CULQI_ENV_LIVE
        };

        let mut headers = Self::create_base_headers(env);

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