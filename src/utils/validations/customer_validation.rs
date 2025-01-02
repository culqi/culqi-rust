use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{constants::constant, custom_exception::CustomException};

pub struct CustomerValidation;

impl CustomerValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::is_valid_email(&parsed,)?;
        Helpers::validate_parameters_string(
            vec![
                "address_city",
                constant::COUNTRY_CODE,
                "first_name",
                "last_name",
                "phone_number",
            ],
            &parsed,
        )?;

        Helpers::validate_allow_values_string(
            &parsed,
            constant::COUNTRY_CODE,
            constant::ALLOW_VALUES_COUNTRY_CODE,
        )?;

        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        if let Some(_email,) = parsed.get(constant::EMAIL,) {
            Helpers::is_valid_email(&parsed,)?;
        }

        Helpers::validate_range_filter(&parsed, constant::CUSTOMER_KEY,)?;

        Ok((),)
    }

    pub fn update(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        if let Some(address,) = parsed.get("address",) {
            Helpers::validate_string(address, "address",)?;
        }

        if let Some(address_city,) = parsed.get("address_city",) {
            Helpers::validate_string(address_city, "address_city",)?;
        }

        if let Some(country_code,) = parsed.get("country_code",) {
            Helpers::validate_string(country_code, "country_code",)?;
            Helpers::validate_allow_values_string(
                &parsed,
                constant::COUNTRY_CODE,
                constant::ALLOW_VALUES_COUNTRY_CODE,
            )?;
        }

        if let Some(first_name,) = parsed.get("first_name",) {
            Helpers::validate_string(first_name, "first_name",)?;
        }

        if let Some(last_name,) = parsed.get("last_name",) {
            Helpers::validate_string(last_name, "last_name",)?;
        }

        if let Some(phone_number,) = parsed.get("phone_number",) {
            Helpers::validate_string(phone_number, "phone_number",)?;
        }

        Helpers::validate_metadata(&parsed,)?;

        Ok((),)
    }
}
