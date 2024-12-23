use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::CustomException::CustomException;

pub struct OrderValidation;

impl OrderValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        let client_details = parsed
            .get("client_details",)
            .and_then(Value::as_object,)
            .ok_or(CustomException::new("client_details is missing or invalid",),)?;
        let first_name = client_details
            .get("first_name",)
            .and_then(Value::as_str,)
            .ok_or(CustomException::new("first name is empty",),)?;
        let last_name = client_details
            .get("last_name",)
            .and_then(Value::as_str,)
            .ok_or(CustomException::new("last name is empty",),)?;
        let phone_number = client_details
            .get("phone_number",)
            .and_then(Value::as_str,)
            .ok_or(CustomException::new("phone_number is empty",),)?;
        let email = client_details
            .get("email",)
            .and_then(Value::as_str,)
            .ok_or(CustomException::new("email is invalid",),)?;

        if first_name.is_empty() || last_name.is_empty() || phone_number.is_empty() {
            return Err(CustomException::new(
                "Names or phone number cannot be empty",
            ),);
        }

        if !Helpers::is_valid_email(email,) {
            return Err(CustomException::new("Invalid email.",),);
        }

        let currency_code = parsed
            .get("currency_code",)
            .and_then(Value::as_str,)
            .ok_or(CustomException::new("currency_code is missing",),)?;
        Helpers::validate_currency_code(currency_code,)?;

        let amount_obj = parsed
            .get("amount",)
            .and_then(Value::as_i64,)
            .ok_or(CustomException::new("Amount not found",),)?;
        Helpers::validate_amount_value(&amount_obj.to_string(),)?;

        let expiration_date =
            parsed
                .get("expiration_date",)
                .and_then(Value::as_i64,)
                .ok_or(CustomException::new(
                    "expiration_date is missing or not a valid integer",
                ),)?;
        if !Helpers::is_future_date(expiration_date,) {
            return Err(CustomException::new(
                "expiration_date must be a future date.",
            ),);
        }

        Ok((),)
    }
}
