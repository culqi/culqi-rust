use super::cardValidation::CardValidation;
use super::chargeValidation::ChargeValidation;
use super::customerValidation::CustomerValidation;
use super::helpers::Helpers;
use super::orderValidation::OrderValidation;
use super::planValidation::PlanValidation;
use super::refundValidation::RefundValidation;
use super::subscriptionValidation::SubscriptionValidation;
use super::tokenValidation::TokenValidation;
use anyhow::Result;
use crate::utils::{urls::{ORDER_URL, TOKEN_URL, TOKEN_YAPE_URL}, CustomException::CustomException};

pub struct ValidateIfAction;

impl ValidateIfAction {
    pub fn validate_class(action: &str, body: &str) -> Result<(String, u16), (String)> {
        if action == TOKEN_URL {
            match TokenValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        if action == "charges" {
            match ChargeValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        if action == "cards" {
            match CardValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        if action == "customers" {
            match CustomerValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        if action.contains("plans") {
            match PlanValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        if action == "refunds" {
            match RefundValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        if action == ORDER_URL {
            match OrderValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        if action == TOKEN_YAPE_URL {
            return Ok(("Validation succeeded".to_string(), 200));
        }

        let error = CustomException::new("Invalid action specified");
        return Err(error.to_string());
    }

    pub fn validate_update_class(action: &str, body: &str) -> Result<(String, u16),(String)> {
        if action.contains("plans") {
            match PlanValidation::update(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::update(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        let error = CustomException::new("Invalid action specified");
        return Err(error.to_string());
    }

    pub fn validate_all_class(action: &str, query: &str) -> Result<(String, u16), (String)> {
        if action.contains("plans") {
            match PlanValidation::list(query) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::list(query) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        Ok(("Validation succeeded".to_string(), 200))
    }

    pub fn validate_id_class(action: &str, query: &str) -> Result<(String, u16), (String)> {
        if action.contains("plans") {
            match Helpers::validate_id(query, "pln") {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        if action.contains("subscriptions") {
            match Helpers::validate_id(query, "sxn") {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        Ok(("Validation succeeded".to_string(), 200))
    }
}
