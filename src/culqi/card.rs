use crate::{
    client::Client,
    utils::{response::handle_response, urls::CARD_URL},
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize,)]
pub struct Card {}
impl Card {
    pub async fn create<T: Serialize,>(
        client: &Client,
        card_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(CARD_URL, card_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(CARD_URL, id, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(CARD_URL, params, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        card_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.patch(CARD_URL, id, card_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn delete(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.delete(CARD_URL, id, custom_headers,).await;
        handle_response(result,).await
    }
}
