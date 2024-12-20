use chrono::{Duration, Utc};
use serde_json::json;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn create_order_request() -> HashMap<String, serde_json::Value> {
    let expiration_date = Utc::now() + Duration::days(1);
    let expiration_timestamp = expiration_date.timestamp();
    let timestamp = Utc::now().timestamp_millis();
    let order_number = format!("#pedido-{}", timestamp);

    let mut order_request = HashMap::new();
    //order_request.insert("amount".to_string(), json!(10000));
    order_request.insert("currency_code".to_string(), json!("PEN"));
    order_request.insert("description".to_string(), json!("Venta de prueba"));
    order_request.insert("order_number".to_string(), json!(order_number));
    order_request.insert("expiration_date".to_string(), json!(expiration_timestamp));
    order_request.insert("confirm".to_string(), json!(true));

    let mut client_details = HashMap::new();
    client_details.insert("first_name".to_string(), json!("Brando"));
    client_details.insert("last_name".to_string(), json!("Carquin"));
    client_details.insert("email".to_string(), json!("brando.carquin@culqi.com"));
    client_details.insert("phone_number".to_string(), json!("+51948747421"));

    order_request.insert("client_details".to_string(), json!(client_details));

    let mut metadata = HashMap::new();
    metadata.insert("dni".to_string(), json!("12345678"));
    order_request.insert("metadata".to_string(), json!(metadata));

    order_request
}

#[allow(dead_code)]
pub const LIST_ORDER_REQUEST: &str = r#"{
    "limit": 1
}"#;