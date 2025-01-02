use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{constant, errors::error, plan},
    custom_exception::CustomException,
};

pub struct PlanValidation;

impl PlanValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_parameters_numeric(
            vec![
                "amount",
                "interval_unit_time",
                "interval_count",
                "interval_count",
            ],
            &parsed,
        )?;

        Helpers::validate_parameters_string(
            vec!["name", "short_name", "description", "currency"],
            &parsed,
        )?;

        Helpers::validate_allow_values_string(
            &parsed,
            "currency",
            constant::ALLOW_VALUES_CURRENCY_CODE,
        )?;

        Helpers::validate_allow_values_numeric(
            &parsed,
            "interval_unit_time",
            plan::ALLOW_VALUES_INTERVAL_UNIT_TIME,
        )?;

        if let Some(initial_cycles,) = parsed.get("initial_cycles",) {
            Helpers::validate_parameters_numeric(
                vec!["count", "amount", "interval_unit_time"],
                initial_cycles,
            )?;

            initial_cycles["has_initial_charge"].as_bool().ok_or_else(|| {
                CustomException::new(
                    "El campo 'initial_cycles.has_initial_charge' es inválido o está vacío.",
                )
            },)?;
        } else {
            return Err(CustomException::new(&error::not_present_parameter(
                "initial_cycles",
            ),),);
        }

        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_allow_values_numeric(
            &parsed,
            constant::STATUS,
            plan::ALLOW_VALUES_STATUS,
        )?;

        if let Some(amount,) = parsed.get("amount",) {
            Helpers::validate_numeric(amount, "amount",)?;
        }

        if let Some(min_amount,) = parsed.get("min_amount",) {
            Helpers::validate_numeric(min_amount, "min_amount",)?;
        }

        if let Some(max_amount,) = parsed.get("max_amount",) {
            Helpers::validate_numeric(max_amount, "max_amount",)?;
        }

        if let Some(limit,) = parsed.get(constant::LIMIT,) {
            Helpers::validate_numeric(limit, constant::LIMIT,)?;
        }

        Helpers::validate_date_filter(
            &parsed,
            constant::CREATION_DATE_FROM,
            constant::CREATION_DATE_TO,
        )?;
        Helpers::validate_range_filter(&parsed, constant::PLAN_KEY,)?;

        Ok((),)
    }

    pub fn update(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        if let Some(name,) = parsed.get("name",) {
            Helpers::validate_string(name, "name",)?;
        }

        if let Some(short_name,) = parsed.get("short_name",) {
            Helpers::validate_string(short_name, "short_name",)?;
        }

        if let Some(description,) = parsed.get("description",) {
            Helpers::validate_string(description, "description",)?;
        }

        if parsed.get("status",).is_some() {
            Helpers::validate_allow_values_numeric(
                &parsed,
                constant::STATUS,
                plan::ALLOW_VALUES_STATUS,
            )?;
        }

        if let Some(image,) = parsed.get("image",) {
            Helpers::validate_string(image, "image",)?;
        }

        Helpers::validate_metadata(&parsed,)?;

        Ok((),)
    }
}
