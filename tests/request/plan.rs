use std::collections::HashMap;

use chrono::Utc;
use serde_json::{json, Value};

#[allow(dead_code)]
pub fn create_plan_request() -> HashMap<String, Value,> {
    let timestamp = Utc::now().timestamp();
    let name = format!("Plan-{}", timestamp);
    let description = format!("Descripción generada - {}", timestamp);
    let short_name = format!("short-{}", timestamp);

    let mut plan_request = HashMap::new();
    plan_request.insert("short_name".to_string(), json!(short_name),);
    plan_request.insert("description".to_string(), json!(description),);
    plan_request.insert("amount".to_string(), json!(300),);
    plan_request.insert("currency".to_string(), json!("PEN"),);
    plan_request.insert("interval_unit_time".to_string(), json!(1),);
    plan_request.insert("interval_count".to_string(), json!(1),);

    let mut initial_cycles = HashMap::new();
    initial_cycles.insert("count".to_string(), json!(1),);
    initial_cycles.insert("has_initial_charge".to_string(), json!(true),);
    initial_cycles.insert("amount".to_string(), json!(400),);
    initial_cycles.insert("interval_unit_time".to_string(), json!(1),);

    plan_request.insert("initial_cycles".to_string(), json!(initial_cycles),);
    plan_request.insert("name".to_string(), json!(name),);

    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), json!("value"),);
    plan_request.insert("metadata".to_string(), json!(metadata),);

    plan_request
}

#[allow(dead_code)]
pub fn request_plan_update() -> Value {
    let timestamp = Utc::now().timestamp();
    let name = format!("Plan-{}", timestamp);
    let description = format!("Descripción generada - {}", timestamp);
    let short_name = format!("short-{}", timestamp);

    json!({
        "short_name": short_name,
        "description": description,
        "name": name
    })
}

#[allow(dead_code)]
pub fn request_plan_all() -> Value {
    json!({
        "limit": 1,
        "status": 1
    })
}
