use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::CustomException::CustomException;

pub struct CardValidation;

impl CardValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        if let Some(source_id,) = parsed.get("customer_id",).and_then(Value::as_str,) {
            Helpers::validate_string_start(source_id, "cus",)?;
        } else {
            return Err(CustomException::new(
                "customer_id not found or is not a string",
            ),);
        }

        if let Some(source_id,) = parsed.get("token_id",).and_then(Value::as_str,) {
            Helpers::validate_string_start(source_id, "tkn",)?;
        } else {
            return Err(CustomException::new(
                "token_id not found or is not a string",
            ),);
        }

        Ok((),)
    }
}
