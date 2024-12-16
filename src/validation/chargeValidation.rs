use crate::utils::CustomException::CustomException;
use serde_json::Value;

use super::helpers::Helpers;

pub struct ChargeValidation;

impl ChargeValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let parsed: Value = serde_json::from_str(body)?;

        if let Some(email) = parsed.get("email").and_then(Value::as_str) {
            if !Helpers::is_valid_email(email) {
                return Err(CustomException::new("Invalid email."));
            }
        } else {
            return Err(CustomException::new("Email not found or is not a string"));
        }

        // Here you might need to handle the type of amountObj as it could be varied
        if let Some(amount_obj) = parsed.get("amount").and_then(Value::as_str) {
            Helpers::validate_amount_value(&amount_obj)?;
        }

        if let Some(currency_code) = parsed.get("currency_code").and_then(Value::as_str) {
            Helpers::validate_currency_code(currency_code)?;
        }

        if let Some(source_id) = parsed.get("source_id").and_then(Value::as_str) {
            if source_id.starts_with("tkn") {
                Helpers::validate_string_start(source_id, "tkn")?;
            } else if source_id.starts_with("ype") {
                Helpers::validate_string_start(source_id, "ype")?;
            } else if source_id.starts_with("crd") {
                Helpers::validate_string_start(source_id, "crd")?;
            } else {
                return Err(CustomException::new(
                    "Incorrect format. The format must start with tkn, ype, or crd.",
                ));
            }
        } else {
            return Err(CustomException::new(
                "Source ID not found or is not a string",
            ));
        }

        Ok(())
    }
}
