use crate::client::Client;
use crate::utils::urls::ORDER_URL;
use crate::CustomRejection;
extern crate serde_json;

use anyhow::Result;
use hyper::Body;
use warp::http::Response;
use serde::Serialize;
use serde_json::json;
use warp::{reply, Reply};
#[derive(Debug, Serialize)]
pub struct Order {}
impl Order {
    pub async fn create<T: Serialize>(
        client: &Client,
        order_request: &T,
        custom_header: Option<&str>,
    ) -> Result<warp::reply::Response, warp::Rejection> {
        match client.post(ORDER_URL, order_request, custom_header).await {
            Ok((response_text, status_code)) => {
                Ok(create_warp_response(response_text, status_code))
            }
            Err((error_message, status_code)) => {
                Ok(create_warp_response(error_message, status_code))
            }
        }
    }

    pub async fn get(
        client: &Client,
        id: &str,
        custom_header: Option<&str>,
    ) -> Result<(String, u16)> {
        let response = client.get(ORDER_URL, &id, custom_header).await?;
        Ok(response)
    }

    pub async fn all(
        client: &Client,
        params: &str,
        custom_header: Option<&str>,
    ) -> Result<(String, u16)> {
        let response = client.all(ORDER_URL, params, custom_header).await?;
        Ok(response)
    }

    pub async fn delete(
        client: &Client,
        id: &str,
        custom_header: Option<&str>,
    ) -> Result<(String, u16)> {
        let response = client.delete(ORDER_URL, &id, custom_header).await?;
        Ok(response)
    }
}

fn create_warp_response(body: String, status_code: u16) -> warp::reply::Response {
    let status = warp::http::StatusCode::from_u16(status_code)
        .unwrap_or(warp::http::StatusCode::INTERNAL_SERVER_ERROR);
    warp::reply::with_status(warp::reply::html(body), status).into_response()
}
