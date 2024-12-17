use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use anyhow::Result;
pub struct Client {
    client: reqwest::Client,
    secret_key: String,
}

impl Client {
    pub fn new(secret_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            secret_key,
        }
    }

    pub fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap(),
        );

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        return headers;
    }

    pub async fn post<T: Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<(String, u16)> {
        let url = get_url(path);
        let json_body = serde_json::to_string(body)?;

        let response = self
            .client
            .post(&url)
            .headers(self.get_headers())
            .body(json_body)
            .send()
            .await?;

        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok((body, status_code))
    }
}

fn get_url(path: &str) -> String {
    String::from("https://api.culqi.com/v2") + path
}
