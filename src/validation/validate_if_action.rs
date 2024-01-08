use anyhow::Result;
use super::tokenValidation::TokenValidation;
use super::chargeValidation::ChargeValidation;

pub struct ValidateIfAction;

impl ValidateIfAction {

    pub fn validate_class(action: &str, body: &str) -> Result<(String, u16)>{
        if(action == "tokens") {
            match TokenValidation::create(body) {
                Ok(_) => {
                    return Ok(("Validation succeeded".to_string(), 200));
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if(action == "charges") {
            match ChargeValidation::create(body) {
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
}
