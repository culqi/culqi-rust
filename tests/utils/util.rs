use BrandoCulqi::client::Client;
use hyper::body::to_bytes;
use serde_json::Value;

use crate::config::credentials::{PUBLIC_KEY, RSA_KEY, SECRET_KEY};

pub fn create_client_encrypt() -> Client {
    Client::config(SECRET_KEY, PUBLIC_KEY, Some(RSA_KEY,),)
}

pub fn create_client() -> Client {
    Client::config(SECRET_KEY, PUBLIC_KEY, None,)
}

pub fn assert_status(response: &hyper::Response<hyper::Body,>, expected_status: u16,) {
    let status_code = response.status().as_u16();
    assert_eq!(
        status_code, expected_status,
        "Código de estado esperado {}, pero obtuviste: {} -> Response: {:?} ",
        expected_status, status_code, response
    );
}

pub async fn parse_response_body(response: hyper::Response<hyper::Body,>,) -> Value {
    let body_bytes = to_bytes(response.into_body(),).await.unwrap();
    let body_string = String::from_utf8_lossy(&body_bytes,).to_string();
    println!("Body: {}", body_string);

    match serde_json::from_str(&body_string,) {
        Ok(json,) => json,
        Err(_,) => panic!("Error al parsear la respuesta JSON"),
    }
}
