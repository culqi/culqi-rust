use std::collections::HashMap;

use chrono::{Duration, Utc};
use serde_json::{json, Value};

#[allow(dead_code)]
pub fn create_order_request() -> HashMap<String, serde_json::Value,> {
    let expiration_date = Utc::now() + Duration::days(1,);
    let expiration_timestamp = expiration_date.timestamp();
    let timestamp = Utc::now().timestamp_millis();
    let order_number = format!("#pedido-{}", timestamp);

    let mut order_request = HashMap::new();
    order_request.insert("amount".to_string(), json!(10000),);
    order_request.insert("currency_code".to_string(), json!("PEN"),);
    order_request.insert("description".to_string(), json!("Venta de prueba"),);
    order_request.insert("order_number".to_string(), json!(order_number),);
    order_request.insert("expiration_date".to_string(), json!(expiration_timestamp),);
    order_request.insert("confirm".to_string(), json!(false),); // Es necesario enviar en false para poder confirmar la orden

    let mut client_details = HashMap::new();
    client_details.insert("first_name".to_string(), json!("Brando"),);
    client_details.insert("last_name".to_string(), json!("Carquin"),);
    client_details.insert("email".to_string(), json!("brando.carquin@culqi.com"),);
    client_details.insert("phone_number".to_string(), json!("+51948747421"),);

    order_request.insert("client_details".to_string(), json!(client_details),);

    let mut metadata = HashMap::new();
    metadata.insert("dni".to_string(), json!("12345678"),);
    order_request.insert("metadata".to_string(), json!(metadata),);

    order_request
}

#[allow(dead_code)]
pub fn request_order_all() -> Value {
    json!({
        "limit": 1
    })
}

#[allow(dead_code)]
pub fn update_order_request() -> HashMap<String, serde_json::Value,> {
    let expiration_date = Utc::now() + Duration::days(1,);
    let expiration_timestamp = expiration_date.timestamp();

    let mut order_request = HashMap::new();
    order_request.insert("amount".to_string(), json!(10200),);
    order_request.insert(
        "description".to_string(),
        json!("Venta de prueba ACTUALIZDA"),
    );
    order_request.insert("expiration_date".to_string(), json!(expiration_timestamp),);

    let mut metadata = HashMap::new();
    metadata.insert("dni".to_string(), json!("87654321"),);
    order_request.insert("metadata".to_string(), json!(metadata),);

    order_request
}

#[allow(dead_code)]
pub fn order_type_confirm_request(id: &str,) -> HashMap<String, serde_json::Value,> {
    let mut order_request = HashMap::new();
    order_request.insert("order_id".to_string(), json!(id),);
    order_request.insert("order_types".to_string(), json!(["cuotealo", "cip"]),);

    order_request
}
