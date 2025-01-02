use std::collections::HashMap;

use serde_json::{json, Value};

use crate::utils::util;

#[allow(dead_code)]
pub fn create_customer_request() -> HashMap<String, Value,> {
    let email = util::generate_email();

    let mut customer_request = HashMap::new();
    customer_request.insert("first_name".to_string(), json!("Brando"),);
    customer_request.insert("last_name".to_string(), json!("Carquin"),);
    customer_request.insert("email".to_string(), json!(email),);
    customer_request.insert("address".to_string(), json!("San Francisco Bay Area"),);
    customer_request.insert("address_city".to_string(), json!("Palo Alto"),);
    customer_request.insert("country_code".to_string(), json!("US"),);
    customer_request.insert("phone_number".to_string(), json!("96739241"),);

    customer_request
}

#[allow(dead_code)]
pub fn update_customer_request() -> Value {
    json!({
        "metadata": {
            "dni": "123",
            "Actualiza": "Campo"
        },
        "address_city": "Address City Update",
    })
}

#[allow(dead_code)]
pub fn request_customer_all() -> Value {
    json!({
        "limit": 1
    })
}
