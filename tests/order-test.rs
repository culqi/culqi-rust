use serde_json::Value;
use BrandoCulqi::client::Client;
use BrandoCulqi::culqi::order::Order;
mod config;
use config::credentials::{SECRET_KEY, PUBLIC_KEY, RSA_KEY, RSA_ID};
use lazy_static::lazy_static;
use warp::Reply;
use warp::hyper::body::to_bytes;

mod request; 
use request::order::{create_order_request,LIST_ORDER_REQUEST};
lazy_static! {
    static ref client: Client = Client::config(SECRET_KEY, PUBLIC_KEY, Some(RSA_KEY));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]

    async fn test_order_create() {
        match Order::create(&client, &create_order_request(), None).await {
            Ok((body, status_code)) => {
                println!("Respuesta de la orden (status code {}): {}", status_code, body);
                let response_json: Value = match serde_json::from_str(&body) {
                    Ok(json) => json,
                    Err(_) => {
                        panic!("Error al parsear la respuesta JSON");
                    }
                };
    
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err((error_body, error_status)) => {
                println!("Error al crear la orden: {} (Código de estado: {})", error_body, error_status);
                panic!("La prueba falló debido a un error al crear la orden");
            }
        }
    }
/*
    #[tokio::test]
    async fn test_order_create_encrypt() {
        const CUSTOM_HEADERS: &str = r#"{
            "x-culqi-rsa-id": "{RSA_ID}"
        }"#;

        let custom_headers = CUSTOM_HEADERS.replace("{RSA_ID}", RSA_ID);

        match Order::create(&client, &create_order_request(), Some(&custom_headers)).await {
            Ok((response_text, status_code)) => {
                println!("Orden encrypt creada exitosamente con status code: {}", status_code);
                println!("Respuesta de la order encrypt : {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(e) => {
                println!("Error al crear la order encrypt: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_order_get() {
        let order = Order::create(&client, &create_order_request(), None).await;
        match order {
            Ok((response_text, _status_code)) => {
                println!("Respuesta crear Orden: {}", response_text);
                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");
                if let Some(order_id) = response_json["id"].as_str() {
                    println!("ID del order: {}", order_id);
                    match Order::get(&client, order_id, None).await {
                        Ok((response_text, _status_code)) => {
                            println!("Respuesta del GET order: {}", response_text);
                            assert_eq!(response_json["object"], "order");
                            assert!(
                                response_json["id"].is_string(),
                                "El campo 'id' no es una cadena"
                            );
                        }
                        Err(e) => eprintln!("Error al obtener el order: {}", e),
                    }
                } else {
                    eprintln!("No se encontró el 'id' en la respuesta del servidor");
                }
            }
            Err(e) => eprintln!("Error al crear el order: {}", e),
        }
    }

    #[tokio::test]
    async fn test_order_list() {
        match Order::all(&client, LIST_ORDER_REQUEST, None).await {
            Ok((response_text, status_code)) => {
                println!(
                    "Token Encrypt creado exitosamente con status code: {}",
                    status_code
                );
                println!("Respuesta del order: {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser 'object'"
                );
            }
            Err(e) => {
                println!("Error al crear el order encriptado: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_order_delete() {
        let order = Order::create(&client, &create_order_request(), None).await;
        match order {
            Ok((response_text, _status_code)) => {
                println!("Respuesta crear Orden: {}", response_text);
                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");
                if let Some(order_id) = response_json["id"].as_str() {
                    println!("ID del order: {}", order_id);
                    match Order::delete(&client, order_id, None).await {
                        Ok((_response_text, status_code)) => {
                            println!("Respuesta del DELETE order | status_code: {}", status_code);
                            assert_eq!(status_code, 204);
                        }
                        Err(e) => eprintln!("Error al obtener el order: {}", e),
                    }
                } else {
                    eprintln!("No se encontró el 'id' en la respuesta del servidor");
                }
            }
            Err(e) => eprintln!("Error al crear el order: {}", e),
        }
    }
*/}
