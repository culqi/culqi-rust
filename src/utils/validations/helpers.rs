use chrono::{TimeZone, Utc};
use regex::Regex;
use serde_json::Value;

use crate::utils::{
    constants::{constant, errors::error, regex_pattern},
    custom_exception::CustomException,
};

pub struct Helpers;

impl Helpers {
    pub fn validate_field(
        parsed: &Value,
        key: &str,
        expected_prefix: &str,
    ) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(key,).and_then(|v| v.as_str(),) {
            Self::validate_start(value, expected_prefix,)?;
            return Ok((),);
        }

        Err(CustomException::new(&error::validate_key_id(key,),),)
    }

    pub fn validate_allow_values_string(
        parsed: &Value,
        key: &str,
        allowed_values: &[&str],
    ) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(key,).and_then(Value::as_str,) {
            Self::validate_enum_values(value, allowed_values,)?;
        }
        Ok((),)
    }

    pub fn validate_allow_values_numeric(
        parsed: &Value,
        key: &str,
        allowed_values: &[i64],
    ) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(key,).and_then(Value::as_i64,) {
            Self::validate_enum_values_numeric(value, allowed_values,)?;
        }
        Ok((),)
    }

    pub fn validate_enum_values_numeric(
        value: i64,
        allowed_values: &[i64],
    ) -> Result<(), CustomException,> {
        if !allowed_values.contains(&value,) {
            return Err(CustomException::new(&error::validate_allow_values(
                allowed_values,
            ),),);
        }

        Ok((),)
    }

    pub fn validate_enum_values(
        value: &str,
        allowed_values: &[&str],
    ) -> Result<(), CustomException,> {
        if !&allowed_values.contains(&value.trim(),) {
            return Err(CustomException::new(&error::validate_allow_values(
                allowed_values,
            ),),);
        }

        Ok((),)
    }

    pub fn validate_start(id: &str, start: &str,) -> Result<(), CustomException,> {
        if !(id.starts_with(&(start.to_string() + "_test_"),)
            || id.starts_with(&(start.to_string() + "_live_"),))
        {
            return Err(CustomException::new(&error::validate_string_start(start,),),);
        }

        Ok((),)
    }

    pub fn validate_numeric(
        value: &serde_json::Value,
        parameter: &str,
    ) -> Result<(), CustomException,> {
        if value.as_f64().is_some() || value.as_i64().is_some() {
            Ok((),)
        } else {
            Err(CustomException::new(&error::validate_type_value(
                parameter, "numeric",
            ),),)
        }
    }

    pub fn validate_string(
        value: &serde_json::Value,
        parameter: &str,
    ) -> Result<(), CustomException,> {
        if let Some(value_str,) = value.as_str() {
            if value_str.is_empty() {
                return Err(CustomException::new(&error::validate_type_value(
                    parameter,
                    "non-empty string",
                ),),);
            }
            Ok((),)
        } else {
            Err(CustomException::new(&error::validate_type_value(
                parameter, "string",
            ),),)
        }
    }

    pub fn validate_date_filter(
        parsed: &Value,
        key_from: &str,
        key_to: &str,
    ) -> Result<(), CustomException,> {
        let date_from = parsed.get(key_from,);
        let date_to = parsed.get(key_to,);

        if let Some(from,) = date_from {
            Self::validate_numeric(from, key_from,)?;
        }

        if let Some(to,) = date_to {
            Self::validate_numeric(to, key_to,)?;
        }

        if let (Some(from,), Some(to,),) = (date_from, date_to,) {
            let from_i64 =
                from.as_i64().or_else(|| from.as_str().and_then(|s| s.parse::<i64>().ok(),),);
            let to_i64 = to.as_i64().or_else(|| to.as_str().and_then(|s| s.parse::<i64>().ok(),),);

            if let (Some(from_i64,), Some(to_i64,),) = (from_i64, to_i64,) {
                if to_i64 < from_i64 {
                    return Err(CustomException::new(error::ERROR_FILTER_DATE,),);
                }
            } else {
                return Err(CustomException::new(
                    "Invalid date values. Ensure they are numeric.",
                ),);
            }
        }

        Ok((),)
    }

    pub fn validate_range_filter(parsed: &Value, key_from: &str,) -> Result<(), CustomException,> {
        let before = parsed.get(constant::BEFORE,);
        let after = parsed.get(constant::AFTER,);

        if let Some(after,) = after {
            Self::validate_start(after.as_str().unwrap_or_default(), key_from,)?;
        }

        if let Some(before,) = before {
            Self::validate_start(before.as_str().unwrap_or_default(), key_from,)?;
        }

        Ok((),)
    }

    pub fn validate_parameters_numeric(
        required_payload: Vec<&str,>,
        parsed: &Value,
    ) -> Result<(), CustomException,> {
        for field in &required_payload {
            if let Some(value,) = parsed.get(*field,) {
                Helpers::validate_numeric(value, field,)?;
            } else {
                return Err(CustomException::new(&error::not_present_parameter(field,),),);
            }
        }

        Ok((),)
    }

    pub fn validate_parameters_string(
        required_payload: Vec<&str,>,
        parsed: &Value,
    ) -> Result<(), CustomException,> {
        for field in &required_payload {
            if let Some(value,) = parsed.get(*field,) {
                Helpers::validate_string(value, field,)?;
            } else {
                return Err(CustomException::new(&error::not_present_parameter(field,),),);
            }
        }

        Ok((),)
    }

    pub fn is_valid_email(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(constant::EMAIL,).and_then(|v| v.as_str(),) {
            let re = Regex::new(regex_pattern::EMAIL,).unwrap();
            if !re.is_match(value,) {
                return Err(CustomException::new(error::INVALID_EMAIL,),);
            }
            Ok((),)
        } else {
            Err(CustomException::new(&error::not_present_parameter(
                constant::EMAIL,
            ),),)
        }
    }

    pub fn is_future_date(parsed: &Value,) -> Result<(), CustomException,> {
        let expiration_date = parsed
            .get(constant::EXPIRATION_DATE,)
            .and_then(|v| v.as_i64(),)
            .ok_or_else(|| {
                CustomException::new("expiration_date is missing or not a valid integer",)
            },)?;

        let exp_date = Utc
            .timestamp_opt(expiration_date, 0,)
            .single()
            .ok_or_else(|| CustomException::new("Invalid expiration_date timestamp",),)?;

        if exp_date < Utc::now() {
            return Err(CustomException::new(error::INVALID_EXPIRATION_FUTURE_DATE,),);
        }

        Ok((),)
    }

    pub fn is_valid_card_numer(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(constant::CARD_NUMBER,) {
            if let Some(card_number,) = value.as_str() {
                let re = Regex::new(regex_pattern::CARD_NUMBER,).unwrap();
                if !re.is_match(card_number,) {
                    return Err(CustomException::new(error::INVALID_CARD_NUMBER,),);
                }
                Ok((),)
            } else {
                Err(CustomException::new(&error::validate_key_id(
                    constant::CARD_NUMBER,
                ),),)
            }
        } else {
            Err(CustomException::new(&error::not_present_parameter(
                constant::CARD_NUMBER,
            ),),)
        }
    }

    pub fn is_valid_cvv(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(constant::CVV,) {
            if let Some(cvv,) = value.as_str() {
                let re = Regex::new(regex_pattern::CVV,).unwrap();
                if !re.is_match(cvv,) {
                    return Err(CustomException::new(error::INVALID_CVV,),);
                }
                Ok((),)
            } else {
                Err(CustomException::new(
                    &error::validate_key_id(constant::CVV,),
                ),)
            }
        } else {
            Err(CustomException::new(&error::not_present_parameter(
                constant::CVV,
            ),),)
        }
    }

    pub fn is_valid_month(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(constant::EXPIRATION_MONTH,) {
            if let Some(month,) = value.as_str() {
                let re = Regex::new(regex_pattern::MONTH,).unwrap();
                if !re.is_match(month,) {
                    return Err(CustomException::new(error::INVALID_MONTH,),);
                }
                Ok((),)
            } else {
                Err(CustomException::new(&error::validate_key_id(
                    constant::EXPIRATION_MONTH,
                ),),)
            }
        } else {
            Err(CustomException::new(&error::not_present_parameter(
                constant::EXPIRATION_MONTH,
            ),),)
        }
    }

    pub fn is_valid_year(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(constant::EXPIRATION_YEAR,) {
            if let Some(year,) = value.as_str() {
                let re = Regex::new(regex_pattern::YEAR,).unwrap();
                if !re.is_match(year,) {
                    return Err(CustomException::new(error::INVALID_YEAR,),);
                }
                Ok((),)
            } else {
                Err(CustomException::new(&error::validate_key_id(
                    constant::EXPIRATION_YEAR,
                ),),)
            }
        } else {
            Err(CustomException::new(&error::not_present_parameter(
                constant::EXPIRATION_YEAR,
            ),),)
        }
    }

    pub fn validate_currency_code(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(currency_code,) = parsed.get(constant::CURRENCY_CODE,).and_then(|v| v.as_str(),)
        {
            if !constant::ALLOW_VALUES_CURRENCY_CODE.contains(&currency_code,) {
                return Err(CustomException::new(error::INVALID_CURRENCY_CODE,),);
            }
        } else {
            return Err(CustomException::new(&error::not_present_parameter(
                constant::CURRENCY_CODE,
            ),),);
        }

        Ok((),)
    }

    pub fn validate_parameters_start(
        expected_prefixs: &[&str],
        value: &str,
        parsed: &Value,
    ) -> Result<(), CustomException,> {
        if let Some(value,) = parsed.get(value,).and_then(Value::as_str,) {
            let expected_prefix = &value.chars().take(3,).collect::<String>();
            if expected_prefixs.contains(&expected_prefix.as_str(),) {
                Self::validate_start(value, expected_prefix,)?;
                return Ok((),);
            }
            Err(CustomException::new(&error::validate_allow_values(
                expected_prefixs,
            ),),)
        } else {
            Err(CustomException::new(&error::not_present_parameter(value,),),)
        }
    }

    pub fn extract_value_from_string<'a,>(
        path: &'a str,
        start_prefix: &str,
        end_prefix: &str,
    ) -> &'a str {
        path.strip_prefix(start_prefix,)
            .and_then(|rest| rest.strip_suffix(end_prefix,),)
            .unwrap_or("",)
    }

    pub fn is_path_match(template: &str, actual: &str,) -> bool {
        let pattern = template.replace("{:id}", r"[^/]+",);
        let re = Regex::new(&format!("^{}$", pattern),).unwrap();
        re.is_match(actual,)
    }

    pub fn validate_metadata(parsed: &Value,) -> Result<(), CustomException,> {
        if let Some(metadata,) = parsed.get(constant::METADATA,) {
            if !metadata.is_object() {
                return Err(CustomException::new(&error::validate_type_value(
                    constant::METADATA,
                    "object",
                ),),);
            }
        }
        Ok((),)
    }

    pub fn validate_metadata_str(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        Self::validate_metadata(&parsed,)?;
        Ok((),)
    }
}
