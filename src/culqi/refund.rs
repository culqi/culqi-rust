use crate::{
    client::Client,
    utils::{response::handle_response, urls::REFUND_URL},
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize,)]
pub struct Refund {}
impl Refund {
    pub async fn create<T: Serialize,>(
        client: &Client,
        refund_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(REFUND_URL, refund_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(REFUND_URL, id, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(REFUND_URL, params, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        refund_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.patch(REFUND_URL, id, refund_request, custom_headers,).await;
        handle_response(result,).await
    }
}
