use regex::Regex;
use std::collections::HashMap;
use chrono::{Local, Datelike, NaiveDate};
use serde_json::Value;

use super::{helpers::Helpers, CustomException::CustomException};

pub struct SubscriptionValidation;

impl SubscriptionValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let parsed: Value = serde_json::from_str(body)?;

        if let Some(source_id) = parsed.get("card_id").and_then(Value::as_str) {
                Helpers::validate_string_start(source_id, "crd")?;
        } else {
            return Err(CustomException::new("card_id not found or is not a string"));
        }
       
        if let Some(source_id) = parsed.get("plan_id").and_then(Value::as_str) {
                Helpers::validate_string_start(source_id, "pln")?;
        } else {
            return Err(CustomException::new("plan_id not found or is not a string"));
        }

        Ok(())
    }
}