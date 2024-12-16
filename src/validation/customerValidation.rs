use crate::utils::CustomException::CustomException;
use serde_json::Value;

use super::helpers::Helpers;

pub struct CustomerValidation;

impl CustomerValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let parsed: Value = serde_json::from_str(body)?;

        let first_name = parsed
            .get("first_name")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("first name is empty."))?;
        if first_name.is_empty() {
            return Err(CustomException::new("first name is empty."));
        }

        let last_name = parsed
            .get("last_name")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("last name is empty."))?;
        if last_name.is_empty() {
            return Err(CustomException::new("last name is empty."));
        }

        let address = parsed
            .get("address")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("address is empty."))?;
        if address.is_empty() {
            return Err(CustomException::new("address is empty."));
        }

        let address_city = parsed
            .get("address_city")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("address_city is empty."))?;
        if address_city.is_empty() {
            return Err(CustomException::new("address_city is empty."));
        }

        // let phone_number = parsed.get("phone_number").and_then(Value::as_str).ok_or(CustomException::new("Invalid 'phone_number'. It should be a string."))?;
        // Assuming phone_number is always a string in Rust, no instanceof check is needed.

        let country_codes = Helpers::get_country_codes();
        let country_code = parsed
            .get("country_code")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("Country code not found"))?;
        Helpers::validate_value(country_code, &country_codes)?;

        let email = parsed
            .get("email")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("Email not found"))?;
        if !Helpers::is_valid_email(email) {
            return Err(CustomException::new("Invalid email."));
        }

        Ok(())
    }
}
