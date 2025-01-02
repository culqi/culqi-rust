use crate::{
    client::Client,
    utils::{
        response::handle_response,
        urls::{CHARGE_CONFIRM_URL, CHARGE_URL, get_url_replace_id},
    },
};
extern crate serde_json;
use anyhow::Result;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Serialize,)]
pub struct Charge {}
impl Charge {
    pub async fn create<T: Serialize,>(
        client: &Client,
        charge_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.post(CHARGE_URL, charge_request, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.get(CHARGE_URL, id, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn all<T: Serialize,>(
        client: &Client,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.all(CHARGE_URL, params, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn patch<T: Serialize,>(
        client: &Client,
        id: &str,
        charge_request: &T,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let result = client.patch(CHARGE_URL, id, charge_request, custom_headers,).await;
        return handle_response(result,).await;
    }

    pub async fn capture(
        client: &Client,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<warp::reply::Response, warp::Rejection,> {
        let path = get_url_replace_id(CHARGE_CONFIRM_URL, id,);
        let empty_body = json!({});
        let result = client.post(&path, &Some(empty_body,), custom_headers,).await;
        return handle_response(result,).await;
    }
}
