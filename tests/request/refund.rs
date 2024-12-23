use std::collections::HashMap;

use serde_json::{json, Value};

use super::charge::create_charge_request;
use crate::utils::charge;

#[allow(dead_code)]
pub async fn create_refund_request() -> HashMap<String, Value,> {
    println!(" Crear Cargo ->");
    let charge_id: String = charge::create_charge().await;
    let charge = create_charge_request().await;

    println!("Request Refund ->");
    let mut refund_request = HashMap::new();
    refund_request.insert("charge_id".to_string(), json!(charge_id),);
    refund_request.insert("amount".to_string(), json!(charge.get("amount")),);
    refund_request.insert("reason".to_string(), json!("fraudulento"),);

    refund_request
}

#[allow(dead_code)]
pub async fn update_refund_request() -> Value {
    json!({
        "metadata": {
            "dni": "71701978"
        },
    })
}

#[allow(dead_code)]
pub fn request_refund_all() -> Value {
    json!({
        "limit": 1
    })
}
