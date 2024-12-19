use crate::utils::urls::{TOKEN_URL,TOKEN_YAPE_URL};
use crate::client::Client;
extern crate serde_json;

use serde::Serialize;
use anyhow::Result;
use warp::reply::Reply;

#[derive(Debug, Serialize)]
pub struct Token {
}

impl Token {
    pub async fn create<T: Serialize>(client: &Client, token_request: &T, custom_header: Option<&str>)  -> Result<impl Reply, warp::Rejection> {
        let response = client.post(TOKEN_URL, token_request, custom_header).await?;
        Ok(response)
    }

    pub async fn get(client: &Client, id: &str, custom_header: Option<&str>) -> Result<(String, u16)> {
        let response = client.get(TOKEN_URL, id, custom_header).await?;
        Ok(response)
    }

    pub async fn all(client: &Client, params: &str, custom_header: Option<&str>) -> Result<(String, u16)> {
        let response = client.all(TOKEN_URL, params, custom_header).await?;
        Ok(response)
    }

    pub async fn patch<T: Serialize>(client: &Client, id: &str, token_request: &T, custom_header: Option<&str>) -> Result<(String, u16)> {
        let response = client.patch(TOKEN_URL, id, token_request, custom_header).await?;
        Ok(response)
    }

    pub async fn yape<T: Serialize>(client: &Client, token_request: &T, custom_header: Option<&str>) -> Result<impl Reply, warp::Rejection> {
        let response = client.post(TOKEN_YAPE_URL, token_request, custom_header).await?;
        Ok(response)
    }
}