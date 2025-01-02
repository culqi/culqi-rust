use crate::{
    client::Client,
    utils::{
        response::handle_response,
        urls::{get_url_replace_id, ORDER_CONFIRM_TYPE_URL, ORDER_CONFIRM_URL, ORDER_URL},
    },
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::{json, Value};
use warp;

#[derive(Debug, Serialize,)]
pub struct Order {}
impl Order {
    pub async fn create<T: Serialize,>(
        client: &Client,
        order_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(ORDER_URL, order_request, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(ORDER_URL, &id, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(ORDER_URL, params, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn delete(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.delete(ORDER_URL, &id, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        order_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.patch(ORDER_URL, &id, &order_request, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn confirm(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let path = get_url_replace_id(ORDER_CONFIRM_URL, id,);
        let empty_body = json!({});
        let result = client.post(&path, &Some(empty_body,), custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn type_confirm<T: Serialize,>(
        client: &Client,
        order_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(ORDER_CONFIRM_TYPE_URL, order_request, custom_headers,).await;
        return handle_response(result,).await;
    }
}
