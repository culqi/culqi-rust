extern crate serde_json;
use crate::client::Client;
use serde::Serialize;
use anyhow::{Result};

#[derive(Debug, Serialize)]
pub struct Order {
}

impl Order {
    pub async fn create<T: Serialize>(client: &Client, order_request: &T) -> Result<(String, u16)> {
        let response = client.post("/orders", order_request).await?;
        Ok(response)
    }
}