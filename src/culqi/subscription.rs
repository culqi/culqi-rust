use crate::{
    client::Client,
    utils::{
        response::handle_response,
        urls::{SUBSCRIPTION_CREATE_URL, SUBSCRIPTION_URL},
    },
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize,)]
pub struct Subscription {}
impl Subscription {
    pub async fn create<T: Serialize,>(
        client: &Client,
        subscription_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client
            .post(
                SUBSCRIPTION_CREATE_URL,
                subscription_request,
                custom_headers,
            )
            .await;
        handle_response(result,).await
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(SUBSCRIPTION_URL, id, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(SUBSCRIPTION_URL, params, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        subscription_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result =
            client.patch(SUBSCRIPTION_URL, id, subscription_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn delete(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.delete(SUBSCRIPTION_URL, id, custom_headers,).await;
        handle_response(result,).await
    }
}
