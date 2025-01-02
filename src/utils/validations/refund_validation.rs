use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{constant, errors::error, refund},
    custom_exception::CustomException,
};

pub struct RefundValidation;

impl RefundValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_field(&parsed, constant::CHARGE_ID, constant::CHARGE_KEY,)?;
        Helpers::validate_parameters_numeric(vec!["amount"], &parsed,)?;

        if parsed.get(constant::REASON,).is_some() {
            Helpers::validate_allow_values_string(
                &parsed,
                constant::REASON,
                refund::ALLOW_VALUES_REASON,
            )?;
        } else {
            return Err(CustomException::new(&error::not_present_parameter(
                constant::REASON,
            ),),);
        }

        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_allow_values_string(
            &parsed,
            constant::REASON,
            refund::ALLOW_VALUES_REASON,
        )?;

        Helpers::validate_date_filter(
            &parsed,
            constant::CREATION_DATE_FROM,
            constant::CREATION_DATE_TO,
        )?;

        Ok((),)
    }
}
