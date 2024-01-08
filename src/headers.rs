use reqwest::header::HeaderMap;

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
        headers.insert("x-culqi-client-version", X_CULQI_CLIENT_VERSION.parse().unwrap());

        return headers;
    }
}