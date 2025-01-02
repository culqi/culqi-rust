use std::collections::HashMap;

use serde_json::{json, Value};

use crate::utils::{customer, token};

#[allow(dead_code)]
pub async fn create_card_request() -> HashMap<String, Value,> {
    let token_id: String = token::create_token().await;
    let customer_id: String = customer::create_customer().await;

    let mut card_request = HashMap::new();
    card_request.insert("customer_id".to_string(), json!(customer_id),);
    card_request.insert("token_id".to_string(), json!(token_id),);
    card_request.insert("validate".to_string(), json!(true),);

    let mut metadata = HashMap::new();
    metadata.insert("marca_tarjeta".to_string(), json!("VISA"),);
    card_request.insert("metadata".to_string(), json!(metadata),);

    card_request
}

#[allow(dead_code)]
pub async fn update_card_request() -> Value {
    let token_id: String = token::create_token().await;

    json!({
        "token_id": token_id,
        "metadata": {
            "dni": "123"
        },
    })
}

#[allow(dead_code)]
pub fn request_card_all() -> Value {
    json!({
        "limit": 1,
    })
}
