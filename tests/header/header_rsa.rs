use serde_json::{Value, json};

use crate::config::credentials::RSA_ID;

pub fn get_header_encrypt() -> Value {
    json!({
        "x-culqi-rsa-id": RSA_ID
    })
}
