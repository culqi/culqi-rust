use crate::validation::validate_if_action::ValidateIfAction;
use crate::utils::urls::ORDER_URL;
use crate::client::Client;
extern crate serde_json;

use serde::Serialize;
use anyhow::Result;

#[derive(Debug, Serialize)]
pub struct Order {
}

impl Order {
    pub async fn create<T: Serialize>(client: &Client, order_request: &T) -> Result<(String, u16)> {
        println!("Init Validation");
        let order_request_json = serde_json::to_string(order_request)?;
        if let Err(validation_error) = ValidateIfAction::validate_class(ORDER_URL, &order_request_json) {
            return Ok((validation_error.to_string(), 400)); // Convertimos el CustomException a String con `to_json()`
        }
        println!("Finished Validation");
        let response = client.post(ORDER_URL, order_request, false).await?;
        Ok(response)
    }
}