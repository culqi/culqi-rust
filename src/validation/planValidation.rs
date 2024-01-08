use regex::Regex;
use std::collections::HashMap;
use chrono::{Local, Datelike, NaiveDate};
use serde_json::Value;

use super::{helpers::Helpers, CustomException::CustomException};

pub struct PlanValidation;

impl PlanValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let parsed: Value = serde_json::from_str(body)?;
        let allowed_values = ["dias", "semanas", "meses", "años"];
        let interval = parsed.get("interval").and_then(Value::as_str)
            .ok_or(CustomException::new("Interval not found or is not a string"))?;

        Helpers::validate_value(interval, &allowed_values)?;

        let amount_obj = parsed.get("amount").and_then(Value::as_str)
            .ok_or(CustomException::new("Amount not found"))?;
        Helpers::validate_amount_value(amount_obj)?;

        let currency_code = parsed.get("currency_code").and_then(Value::as_str)
            .ok_or(CustomException::new("Currency code not found or is not a string"))?;
        Helpers::validate_currency_code(currency_code)?;

        Ok(())
    }
}