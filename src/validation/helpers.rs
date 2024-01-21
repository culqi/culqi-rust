extern crate regex;
use std::collections::HashSet;

use chrono::{Utc, TimeZone};
use regex::Regex;

use super::CustomException::CustomException;

pub struct Helpers;

impl Helpers {

    pub fn is_valid_card_number(number: &str) -> bool {
        let re = Regex::new(r"^\d{13,19}$").unwrap();
        re.is_match(number)
    }
    
    pub fn is_valid_email(email: &str) -> bool {
        let re = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
        re.is_match(email)
    }

    pub fn validate_currency_code(currency_code: &str) -> Result<(), CustomException> {
        if currency_code.is_empty() {
            return Err(CustomException::new("Currency code is empty."));
        }
    
        let allowed_values: HashSet<&str> = ["PEN", "USD"].iter().cloned().collect();
        if !allowed_values.contains(currency_code) {
            return Err(CustomException::new("Currency code must be either \"PEN\" or \"USD\"."));
        }
    
        Ok(())
    }

    pub fn validate_string_start(string: &str, start: &str) -> Result<(), CustomException> {
        if !(string.starts_with(&(start.to_string() + "_test_")) || string.starts_with(&(start.to_string() + "_live_"))) {
            return Err(CustomException::new(&format!("Incorrect format. The format must start with {}_test_ or {}_live_", start, start)));
        }
    
        Ok(())
    }
    
    pub fn validate_value(value: &str, allowed_values: &[&str]) -> Result<(), CustomException> {
        if !allowed_values.contains(&value) {
            println!("{}", value); // Similar to System.err.println in Java
            return Err(CustomException::new(&format!("Invalid value. It must be one of {:?}", allowed_values)));
        }
    
        Ok(())
    }
    
    pub fn is_future_date(expiration_date: i64) -> bool {
        let exp_date = Utc.timestamp(expiration_date, 0);
        exp_date > Utc::now()
    }
    
    pub fn validate_date_filter(date_from: &str, date_to: &str) -> Result<(), CustomException> {
        let parsed_date_from = date_from.parse::<i32>()
            .map_err(|_| CustomException::new("Invalid value. Date_from must be an integer."))?;
        let parsed_date_to = date_to.parse::<i32>()
            .map_err(|_| CustomException::new("Invalid value. Date_to must be an integer."))?;
    
        if parsed_date_to < parsed_date_from {
            return Err(CustomException::new("Invalid value. Date_from must be less than Date_to."));
        }
    
        Ok(())
    }
    
    pub fn validate_amount_value(amount_obj: &str) -> Result<(), CustomException> {
        match amount_obj.parse::<i32>() {
            Ok(_) => Ok(()), // If it's a valid integer, no further validation is needed.
            Err(_) => Err(CustomException::new("Invalid 'amount'. It should be an integer or a string representing an integer.")),
        }
    }

    pub fn get_country_codes() -> Vec<&'static str> {
        vec!["AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX", "AZ",
        "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ", "BR", "BS",
        "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN",
        "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ", "DE", "DJ", "DK", "DM", "DO", "DZ", "EC", "EE",
        "EG", "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR", "GA", "GB", "GD", "GE", "GF",
        "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM",
        "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM",
        "JO", "JP", "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC",
        "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK",
        "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA",
        "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR", "NU", "NZ", "OM", "PA", "PE", "PF", "PG",
        "PH", "PK", "PL", "PM", "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW",
        "SA", "SB", "SC", "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS",
        "ST", "SV", "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO",
        "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI",
        "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW"]
    }

}
