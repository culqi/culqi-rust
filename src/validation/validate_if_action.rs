use anyhow::Result;
use super::cardValidation::CardValidation;
use super::customerValidation::CustomerValidation;
use super::orderValidation::OrderValidation;
use super::planValidation::PlanValidation;
use super::refundValidation::RefundValidation;
use super::subscriptionValidation::SubscriptionValidation;
use super::tokenValidation::TokenValidation;
use super::chargeValidation::ChargeValidation;
use super::helpers::Helpers;


pub struct ValidateIfAction;

impl ValidateIfAction {

    pub fn validate_class(action: &str, body: &str) -> Result<(String, u16)>{
        if action == "tokens" {
            match TokenValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if action == "charges" {
            match ChargeValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        
        if action == "cards" {
            match CardValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        
        if action == "customers" {
            match CustomerValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        
        if action.contains("plans") {
            match PlanValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
            
        }
        
        if action == "refunds" {
            match RefundValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        if action == "orders" {
            match OrderValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Err(anyhow::Error::msg("Invalid action specified"))
    }

    pub fn validate_update_class(action: &str, body: &str) -> Result<(String, u16)>{
        if action.contains("plans") {
            match PlanValidation::update(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
            
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::update(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        Err(anyhow::Error::msg("Invalid action specified"))
    }

    pub fn validate_all_class(action: &str, query: &str) -> Result<(String, u16)>{
        if action.contains("plans") {
            match PlanValidation::list(query) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
            
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::list(query) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        Ok(("Validation succeeded".to_string(), 200))
    }

    pub fn validate_id_class(action: &str, query: &str) -> Result<(String, u16)>{
        if action.contains("plans") {
            match Helpers::validate_id(query, "pln") {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
            
        }
        if action.contains("subscriptions") {
            match Helpers::validate_id(query, "sxn") {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        Ok(("Validation succeeded".to_string(), 200))
    }

}
