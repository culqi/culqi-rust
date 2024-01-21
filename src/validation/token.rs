use regex::Regex;
use std::collections::HashMap;
use chrono::{YearMonth, Local};

pub struct Token;

impl Token {
    pub fn create(body: &HashMap<String, String>) -> Result<(), CustomException> {
        if !Helper::is_valid_card_number(body.get("card_number").unwrap()) {
            return Err(CustomException::new("Invalid card number."));
        }

        let cvv_pattern = Regex::new(r"^\d{3,4}$").unwrap();
        if !cvv_pattern.is_match(body.get("cvv").unwrap()) {
            return Err(CustomException::new("Invalid CVV."));
        }

        if !Helper::is_valid_email(body.get("email").unwrap()) {
            return Err(CustomException::new("Invalid email."));
        }

        let month_pattern = Regex::new(r"^(0?[1-9]|1[012])$").unwrap();
        if !month_pattern.is_match(body.get("expiration_month").unwrap()) {
            return Err(CustomException::new("Invalid expiration month."));
        }

        let year_pattern = Regex::new(r"^\d{4}$").unwrap();
        let year = body.get("expiration_year").unwrap().parse::<i32>().unwrap();
        let current_year = Local::now().year();
        if !year_pattern.is_match(body.get("expiration_year").unwrap()) || year < current_year {
            return Err(CustomException::new("Invalid expiration year."));
        }

        let exp_date = YearMonth::parse_from_str(
            &format!("{}-{}", body.get("expiration_year").unwrap(), body.get("expiration_month").unwrap()),
            "%Y-%m",
        ).unwrap();
        if exp_date < YearMonth::now() {
            return Err(CustomException::new("Card has expired."));
        }

        Ok(())
    }
}