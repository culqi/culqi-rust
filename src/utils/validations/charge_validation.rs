use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{card, charge, constant},
    custom_exception::CustomException,
};

pub struct ChargeValidation;

impl ChargeValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        Helpers::is_valid_email(&parsed,)?;

        Helpers::validate_parameters_numeric(vec!["amount"], &parsed,)?;
        Helpers::validate_currency_code(&parsed,)?;
        Helpers::validate_parameters_start(
            charge::ALLOW_VALUES_SOURCE_ID,
            constant::SOURCER_ID,
            &parsed,
        )?;

        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_allow_values_string(
            &parsed,
            constant::CURRENCY_CODE,
            constant::ALLOW_VALUES_CURRENCY_CODE,
        )?;

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

        Helpers::validate_date_filter(
            &parsed,
            constant::CREATION_DATE_FROM,
            constant::CREATION_DATE_TO,
        )?;
        Helpers::validate_range_filter(&parsed, constant::CHARGE_KEY,)?;

        Ok((),)
    }
}
