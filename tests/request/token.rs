use std::collections::HashMap;

use serde_json::{Value, json};

#[allow(dead_code)]
pub fn create_token_request() -> HashMap<String, serde_json::Value,> {
    let mut token_request = HashMap::new();
    token_request.insert("card_number".to_string(), json!("371212121212122"),);
    token_request.insert("cvv".to_string(), json!("2841"),);
    token_request.insert("expiration_month".to_string(), json!("11"),);
    token_request.insert("expiration_year".to_string(), json!("2025"),);
    token_request.insert("email".to_string(), json!("accept@culqi.com"),);

    let mut metadata = HashMap::new();
    metadata.insert("dni".to_string(), json!("12345678"),);
    token_request.insert("metadata".to_string(), json!(metadata),);

    token_request
}

#[allow(dead_code)]
pub fn update_token_request() -> HashMap<String, serde_json::Value,> {
    let mut token_request = HashMap::new();
    token_request.insert(
        "email".to_string(),
        json!("actualiza_brando.carquin@culqi.com"),
    );

    let mut metadata = HashMap::new();
    metadata.insert("dni".to_string(), json!("87654321"),);
    token_request.insert("metadata".to_string(), json!(metadata),);

    token_request
}

#[allow(dead_code)]
pub fn create_token_yape_request() -> HashMap<String, serde_json::Value,> {
    let mut token_yape_request = HashMap::new();
    token_yape_request.insert("otp".to_string(), json!("172663"),);
    token_yape_request.insert("number_phone".to_string(), json!("967395241"),);
    token_yape_request.insert("amount".to_string(), json!("500"),);

    let mut metadata = HashMap::new();
    metadata.insert("dni".to_string(), json!("12345678"),);
    token_yape_request.insert("metadata".to_string(), json!(metadata),);

    token_yape_request
}

#[allow(dead_code)]
pub fn request_token_all() -> Value {
    json!({
        "limit": 1,
        "device_type": "mobile"
    })
}
