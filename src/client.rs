use crate::utils::urls::{BASE_URL, SECURE_URL};
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
pub struct Client {
    client: reqwest::Client,
    secret_key: String,
}

impl Client {
    pub fn new(secret_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            secret_key: secret_key.to_string(),
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
        secure: bool,
    ) -> Result<(String, u16)> {
        let url = get_url(path, secure);
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

fn get_url(path: &str, secure: bool) -> String {
    let base_url = if secure { SECURE_URL } else { BASE_URL };
    format!("{}{}", base_url, path)
}
