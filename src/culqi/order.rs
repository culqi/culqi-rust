use crate::client::Client;
use crate::utils::urls::ORDER_URL;
extern crate serde_json;

use anyhow::Result;
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use warp::{reply, Reply};
#[derive(Debug, Serialize)]
pub struct Order {}

#[derive(Debug)] // Agregar la derivación de Debug
pub struct CustomRejection {
    status: warp::http::StatusCode,
    body: String,
}

impl CustomRejection {
    pub fn from_status_code(status: warp::http::StatusCode, body: String) -> Self {
        Self { status, body }
    }
}

impl warp::reject::Reject for CustomRejection {}

impl Order {
    pub async fn create<T: Serialize>(
        client: &Client,
        order_request: &T,
        custom_header: Option<&str>,
    ) -> Result<warp::reply::Response, warp::Rejection> {
        match client.post(ORDER_URL, order_request, custom_header).await {
            Ok((response_text, status_code)) => {
                // Si la petición es exitosa, retorna la respuesta con su código de estado
                let response = create_warp_response(response_text, status_code);
                Ok(response) // Aquí se retorna un Result<warp::reply::Response, warp::Rejection>
            }
            Err((error_message, status_code)) => {
                let status =
                    StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                Err(warp::reject::custom(CustomRejection::from_status_code(
                    status,
                    error_message,
                )))
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

    if status.is_success() {
        warp::reply::with_status(warp::reply::html(body), status).into_response()
    } else {
        let error_body = json!({ "message": body });
        let warp_response = warp::reply::with_status(warp::reply::json(&error_body), status);
        warp_response.into_response()
    }
}
