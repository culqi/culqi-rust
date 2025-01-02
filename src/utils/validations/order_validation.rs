use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::{
    constants::{constant, errors::error},
    custom_exception::CustomException,
};

pub struct OrderValidation;

impl OrderValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        Helpers::validate_parameters_numeric(vec!["amount"], &parsed,)?;
        Helpers::validate_parameters_string(vec!["order_number", "description"], &parsed,)?;
        Helpers::validate_currency_code(&parsed,)?;
        Helpers::is_future_date(&parsed,)?;

        if let Some(client_details,) = parsed.get("client_details",) {
            Helpers::validate_parameters_string(
                vec!["first_name", "last_name", "phone_number", "email"],
                client_details,
            )?;
            Helpers::is_valid_email(client_details,)?;
        } else {
            return Err(CustomException::new(&error::not_present_parameter(
                "client_details",
            ),),);
        }

        Ok((),)
    }

    pub fn confirm(url: &str,) -> Result<(), CustomException,> {
        let orderId: &str = Helpers::extract_value_from_string(url, "/orders/", "/confirm",);
        Helpers::validate_start(orderId, constant::ORDER_KEY,)?;
        Ok((),)
    }

    pub fn confirm_type(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        Helpers::validate_field(&parsed, constant::ORDER_ID, constant::ORDER_KEY,)?;

        if let Some(order_types,) = parsed.get("order_types",).and_then(|v| v.as_array(),) {
            for order_type in order_types {
                if let Some(order_type_str,) = order_type.as_str() {
                    if !constant::ALLOW_VALUES_ORDER_TYPES.contains(&order_type_str,) {
                        return Err(CustomException::new(&error::validate_allow_values(
                            constant::ALLOW_VALUES_ORDER_TYPES,
                        ),),);
                    }
                } else {
                    return Err(CustomException::new(
                        "Invalid type in 'order_types': expected a string",
                    ),);
                }
            }
        } else {
            return Err(CustomException::new(&error::not_present_parameter(
                "order_types",
            ),),);
        }

        Ok((),)
    }

    pub fn list(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;

        if let Some(_email,) = parsed.get(constant::EMAIL,) {
            Helpers::is_valid_email(&parsed,)?;
        }

        if let Some(min_amount,) = parsed.get("min_amount",) {
            Helpers::validate_numeric(min_amount, "min_amount",)?;
        }

        if let Some(amount,) = parsed.get("amount",) {
            Helpers::validate_numeric(amount, "amount",)?;
        }

        if let Some(max_amount,) = parsed.get("max_amount",) {
            Helpers::validate_numeric(max_amount, "max_amount",)?;
        }

        Helpers::validate_date_filter(
            &parsed,
            constant::CREATION_DATE_FROM,
            constant::CREATION_DATE_TO,
        )?;

        Ok((),)
    }

    pub fn update(body: &str,) -> Result<(), CustomException,> {
        let parsed: Value = serde_json::from_str(body,)?;
        Helpers::is_future_date(&parsed,)?;

        Helpers::validate_metadata(&parsed,)?;

        Ok((),)
    }
}
