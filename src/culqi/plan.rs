use crate::{
    client::Client,
    utils::{
        response::handle_response,
        urls::{PLAN_CREATE_URL, PLAN_URL},
    },
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize,)]
pub struct Plan {}
impl Plan {
    pub async fn create<T: Serialize,>(
        client: &Client,
        plan_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(PLAN_CREATE_URL, plan_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(PLAN_URL, id, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(PLAN_URL, params, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        plan_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.patch(PLAN_URL, id, plan_request, custom_headers,).await;
        handle_response(result,).await
    }

    pub async fn delete(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.delete(PLAN_URL, id, custom_headers,).await;
        handle_response(result,).await
    }
}
