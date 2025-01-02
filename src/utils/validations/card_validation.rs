use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{card, constant},
    custom_exception::CustomException,
};

pub struct CardValidation;

impl CardValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        Helpers::validate_field(&parsed, constant::CUSTOMER_ID, constant::CUSTOMER_KEY,)?;
        Helpers::validate_field(&parsed, constant::TOKEN_ID, constant::TOKEN_KEY,)?;
        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_allow_values_string(
            &parsed,
            card::CARD_BRAND,
            card::ALLOW_VALUES_CARD_BRAND,
        )?;
        Helpers::validate_allow_values_string(
            &parsed,
            card::CARD_TYPE,
            card::ALLOW_VALUES_CARD_TYPE,
        )?;

        Helpers::validate_allow_values_string(
            &parsed,
            constant::COUNTRY_CODE,
            constant::ALLOW_VALUES_COUNTRY_CODE,
        )?;

        Helpers::validate_allow_values_string(
            &parsed,
            constant::DEVICE_TYPE,
            constant::ALLOW_VALUES_DEVICE_TYPE,
        )?;

        Helpers::validate_date_filter(
            &parsed,
            constant::CREATION_DATE_FROM,
            constant::CREATION_DATE_TO,
        )?;
        Helpers::validate_range_filter(&parsed, constant::CARD_KEY,)?;

        Ok((),)
    }

    pub fn update(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        if let Some(_email,) = parsed.get(constant::TOKEN_ID,) {
            Helpers::validate_field(&parsed, constant::TOKEN_ID, constant::TOKEN_KEY,)?;
        }

        Helpers::validate_metadata(&parsed,)?;
        Ok((),)
    }
}
