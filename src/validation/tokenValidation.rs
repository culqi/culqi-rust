use regex::Regex;
use std::collections::HashMap;
use chrono::{Local, Datelike, NaiveDate};
use serde_json::Value;

use super::{helpers::Helpers, CustomException::CustomException};

pub struct TokenValidation;

impl TokenValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let parsed: Value = serde_json::from_str(body)?;
        if let Some(card_number) = parsed.get("card_number").and_then(Value::as_str) {
            if !Helpers::is_valid_card_number(card_number) {
                return Err(CustomException::new("Invalid card number."));
            }
        } else {
            return Err(CustomException::new("Card number not found or is not a string"));
        }

        let cvv_pattern = Regex::new(r"^\d{3,4}$").unwrap();
        if let Some(cvv) = parsed.get("cvv").and_then(Value::as_str) {
            if !cvv_pattern.is_match(cvv) {
                return Err(CustomException::new("Invalid CVV."));
            }
        } else {
            return Err(CustomException::new("Invalid CVV."));
        }

        if let Some(email) = parsed.get("email").and_then(Value::as_str) {
            if !Helpers::is_valid_email(email) {
                return Err(CustomException::new("Invalid email."));
            }
        } else {
            return Err(CustomException::new("Invalid email."));
        }

        let month_regex = Regex::new(r"^(0?[1-9]|1[012])$").unwrap();
        if let Some(month) = parsed.get("expiration_month").and_then(Value::as_str) {
            if !month_regex.is_match(month) {
                return Err(CustomException::new("Invalid expiration month."));
            }
        } else {
            return Err(CustomException::new("Missing expiration month."));
        }

        let year_regex = Regex::new(r"^\d{4}$").unwrap();
        let current_year = Local::now().year();
        if let Some(year_str) = parsed.get("expiration_year").and_then(Value::as_str) {
            if !year_regex.is_match(year_str) || year_str.parse::<i32>().ok() < Some(current_year) {
                return Err(CustomException::new("Invalid expiration year."));
            }
        } else {
            return Err(CustomException::new("Missing expiration year."));
        }

        if let (Some(year), Some(month)) = (parsed.get("expiration_year").and_then(Value::as_str), parsed.get("expiration_month").and_then(Value::as_str)) {
            let exp_date_str = format!("{}-{}-01", year, month);
            if let Ok(exp_date) = NaiveDate::parse_from_str(&exp_date_str, "%Y-%m-%d") {
                if exp_date < Local::now().naive_local().date() {
                    return Err(CustomException::new("Card has expired."));
                }
            } else {
                return Err(CustomException::new("Invalid expiration date format."));
            }
        }
        Ok(())
    }
}