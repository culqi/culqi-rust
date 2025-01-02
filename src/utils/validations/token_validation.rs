use chrono::{Local, NaiveDate};
use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{card, constant, errors::error},
    custom_exception::CustomException,
};

pub struct TokenValidation;

impl TokenValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::is_valid_email(&parsed,)?;
        Helpers::is_valid_card_numer(&parsed,)?;
        Helpers::is_valid_cvv(&parsed,)?;
        Helpers::is_valid_month(&parsed,)?;
        Helpers::is_valid_year(&parsed,)?;

        if let (Some(year,), Some(month,),) = (
            parsed.get(constant::EXPIRATION_YEAR,).and_then(Value::as_str,),
            parsed.get(constant::EXPIRATION_MONTH,).and_then(Value::as_str,),
        ) {
            let exp_date_str = format!("{}-{}-01", year, month);
            if let Ok(exp_date,) = NaiveDate::parse_from_str(&exp_date_str, "%Y-%m-%d",) {
                if exp_date < Local::now().naive_local().date() {
                    return Err(CustomException::new(error::CARD_EXPIRED,),);
                }
            } else {
                return Err(CustomException::new(error::INVALID_EXPIRATION_EXPIRED,),);
            }
        }

        Ok((),)
    }

    pub fn create_yape(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_parameters_string(vec!["amount", "otp", "number_phone"], &parsed,)?;
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
            constant::DEVICE_TYPE,
            constant::ALLOW_VALUES_DEVICE_TYPE,
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

        Ok((),)
    }
}
