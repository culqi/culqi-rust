use BrandoCulqi::client::Client;
use BrandoCulqi::culqi::order::Order;
use std::collections::HashMap;
use serde_json::{json, Value};
mod config;
use config::credentials::SECRET_KEY;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[tokio::test]
    async fn test_order_create() {

        // Crear el cuerpo de la solicitud utilizando la nueva estructura
        let expiration_date = Utc::now() + Duration::days(1);
        let expiration_timestamp = expiration_date.timestamp();
        let timestamp = Utc::now().timestamp_millis();
        let order_number = format!("#pedido-{}", timestamp);

        // Crear el cuerpo de la solicitud utilizando HashMap y serde_json::json!
        let mut order_request = HashMap::new();
        order_request.insert("amount".to_string(), json!(10000));
        order_request.insert("currency_code".to_string(), json!("PEN"));
        order_request.insert("description".to_string(), json!("Venta de prueba"));
        order_request.insert("order_number".to_string(), json!(order_number));
        order_request.insert("expiration_date".to_string(), json!(expiration_timestamp.to_string()));
        order_request.insert("confirm".to_string(), json!(true));

        // Crear los detalles del cliente como un HashMap anidado
        let mut client_details = HashMap::new();
        client_details.insert("first_name".to_string(), json!("Brando"));
        client_details.insert("last_name".to_string(), json!("Carquin"));
        client_details.insert("email".to_string(), json!("brando.carquin@culqi.com"));
        client_details.insert("phone_number".to_string(), json!("+51948747421"));

        // Añadir client_details al cuerpo de la solicitud
        order_request.insert("client_details".to_string(), json!(client_details));

        // Crear los metadatos como un HashMap
        let mut metadata = HashMap::new();
        metadata.insert("dni".to_string(), json!("71702999"));

        // Añadir metadata al cuerpo de la solicitud
        order_request.insert("metadata".to_string(), json!(metadata));

        let client = Client::new(SECRET_KEY);

        match Order::create(&client, &order_request).await {
            Ok((response_text, status_code)) => {
                println!("Orden creada exitosamente con status code: {}", status_code);
                println!("Respuesta de la orden: {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert_eq!(response_json["object"], "order");
                assert!(response_json["id"].is_string(), "El campo 'id' no es una cadena");
            }
            Err(e) => eprintln!("Error al crear la orden: {}", e),
        }
    }
}
