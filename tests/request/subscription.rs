use std::collections::HashMap;

use serde_json::{json, Value};

use crate::utils::{card, plan};

#[allow(dead_code)]
pub async fn create_subscription_request() -> HashMap<String, Value,> {
    let card_id: String = card::create_card().await;
    let plan_id: String = plan::create_plan().await;

    let mut subscription_request = HashMap::new();
    subscription_request.insert("card_id".to_string(), json!(card_id),);
    subscription_request.insert("plan_id".to_string(), json!(plan_id),);
    subscription_request.insert("tyc".to_string(), json!(true),);

    let mut metadata = HashMap::new();
    metadata.insert("marca_tarjeta".to_string(), json!("VISA"),);
    subscription_request.insert("metadata".to_string(), json!(metadata),);

    subscription_request
}

#[allow(dead_code)]
pub async fn update_subscription_request() -> Value {
    let card_id: String = card::create_card().await;

    json!({
        "card_id": card_id,
        "metadata": {
            "dni": "123"
        },
    })
}

#[allow(dead_code)]
pub fn request_subscription_all() -> Value {
    json!({
        "limit": 1
    })
}
