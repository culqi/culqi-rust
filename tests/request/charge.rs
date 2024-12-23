use std::collections::HashMap;

use serde_json::{json, Value};

use crate::utils::token;

#[allow(dead_code)]
pub async fn create_charge_request() -> HashMap<String, Value,> {
    let token_id: String = token::create_token().await;

    let mut charge_request = HashMap::new();
    charge_request.insert("amount".to_string(), json!(10000),);
    charge_request.insert("currency_code".to_string(), json!("PEN"),);
    charge_request.insert("email".to_string(), json!("accept@culqi.com"),);
    charge_request.insert("source_id".to_string(), json!(token_id),);
    charge_request.insert("capture".to_string(), json!(false),);

    let mut antifraud_details = HashMap::new();
    antifraud_details.insert("address".to_string(), json!("Avenida Lima 1234"),);
    antifraud_details.insert("address_city".to_string(), json!("Lima"),);
    antifraud_details.insert("country_code".to_string(), json!("PE"),);
    antifraud_details.insert("first_name".to_string(), json!("culqi"),);
    antifraud_details.insert("last_name".to_string(), json!("core"),);
    antifraud_details.insert("phone_number".to_string(), json!("999777666"),);
    charge_request.insert("antifraud_details".to_string(), json!(antifraud_details),);

    let mut metadata = HashMap::new();
    metadata.insert("documentNumber".to_string(), json!("77723083"),);
    charge_request.insert("metadata".to_string(), json!(metadata),);

    charge_request
}

#[allow(dead_code)]
pub async fn update_charge_request() -> Value {
    let token_id: String = token::create_token().await;

    json!({
        "token_id": token_id,
        "metadata": {
            "dni": "123"
        },
    })
}

#[allow(dead_code)]
pub fn request_charge_all() -> Value {
    json!({
        "limit": 1
    })
}
