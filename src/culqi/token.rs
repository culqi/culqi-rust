use crate::{
    client::Client,
    utils::{
        response::handle_response,
        urls::{TOKEN_URL, TOKEN_YAPE_URL},
    },
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize,)]
pub struct Token {}
impl Token {
    pub async fn create<T: Serialize,>(
        client: &Client,
        token_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(TOKEN_URL, token_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(TOKEN_URL, id, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(TOKEN_URL, params, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        token_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.patch(TOKEN_URL, id, token_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn yape<T: Serialize,>(
        client: &Client,
        token_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(TOKEN_YAPE_URL, token_request, custom_headers,).await;
        handle_response(result,).await
    }
}
