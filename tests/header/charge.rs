use serde_json::{json, Value};

#[allow(dead_code)]
pub fn get_header_charge_recurrent() -> Value {
    json!({
        "X-Charge-Channels": "recurrent"
    })
}
