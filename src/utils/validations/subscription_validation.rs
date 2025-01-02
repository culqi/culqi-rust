use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{constant, subscription},
    custom_exception::CustomException,
};

pub struct SubscriptionValidation;

impl SubscriptionValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_field(&parsed, constant::CARD_ID, constant::CARD_KEY,)?;
        Helpers::validate_field(&parsed, constant::PLAN_ID, constant::PLAN_KEY,)?;

        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_allow_values_numeric(
            &parsed,
            constant::STATUS,
            subscription::ALLOW_VALUES_STATUS,
        )?;

        if let Some(_,) = parsed.get(constant::PLAN_ID,) {
            Helpers::validate_field(&parsed, constant::PLAN_ID, constant::PLAN_KEY,)?;
        }

        if let Some(limit,) = parsed.get(constant::LIMIT,) {
            Helpers::validate_numeric(limit, constant::LIMIT,)?;
        }

        Helpers::validate_date_filter(
            &parsed,
            constant::CREATION_DATE_FROM,
            constant::CREATION_DATE_TO,
        )?;
        Helpers::validate_range_filter(&parsed, constant::SUBSCRIPTION_KEY,)?;

        Ok((),)
    }

    pub fn update(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        if let Some(_,) = parsed.get("card_id",) {
            Helpers::validate_field(&parsed, constant::CARD_ID, constant::CARD_KEY,)?;
        }

        Helpers::validate_metadata(&parsed,)?;

        Ok((),)
    }
}
