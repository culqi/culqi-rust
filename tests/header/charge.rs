use serde_json::{json, Value};

pub fn get_header_charge_recurrent() -> Value {
    json!({
        "X-Charge-Channels": "recurrent"
    })
}
