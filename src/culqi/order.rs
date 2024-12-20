use crate::utils::urls::ORDER_URL;
use crate::client::Client;
extern crate serde_json;

use serde::Serialize;
use anyhow::Result;

#[derive(Debug, Serialize)]
pub struct Order {
}

impl Order {
    pub async fn create<T: Serialize>(client: &Client, order_request: &T, custom_header: Option<&str>) -> Result<(String, u16), (String, u16)> {
        return client.post(ORDER_URL, order_request, custom_header).await;
    }

    pub async fn get(client: &Client, id: &str, custom_header: Option<&str>) -> Result<(String, u16)> {
        let response = client.get(ORDER_URL, &id, custom_header).await?;
        Ok(response)
    }

    pub async fn all(client: &Client, params: &str, custom_header: Option<&str>) -> Result<(String, u16)> {
        let response = client.all(ORDER_URL, params, custom_header).await?;
        Ok(response)
    }

    pub async fn delete(client: &Client, id: &str, custom_header: Option<&str>) -> Result<(String, u16)> {
        let response = client.delete(ORDER_URL, &id, custom_header).await?;
        Ok(response)
    }
}