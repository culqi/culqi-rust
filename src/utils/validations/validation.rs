use super::{
    card_validation::CardValidation, charge_validation::ChargeValidation,
    customer_validation::CustomerValidation, helpers::Helpers, order_validation::OrderValidation,
    plan_validation::PlanValidation, refund_validation::RefundValidation,
    subscription_validation::SubscriptionValidation, token_validation::TokenValidation,
};
use crate::utils::{constants::constant, custom_exception::CustomException, urls};

pub struct Validation;

impl Validation {
    pub fn create(url: &str, body: &str,) -> Result<(String, u16,), (String, u16,),> {
        let result = match url {
            urls::CARD_URL => CardValidation::create(body,),
            urls::CHARGE_URL => ChargeValidation::create(body,),
            urls::TOKEN_URL => TokenValidation::create(body,),
            urls::TOKEN_YAPE_URL => TokenValidation::create_yape(body,),
            urls::CUSTOMER_URL => CustomerValidation::create(body,),
            urls::ORDER_URL => OrderValidation::create(body,),
            _ if Helpers::is_path_match(urls::ORDER_CONFIRM_URL, url,) => {
                OrderValidation::confirm(url,)
            }
            urls::ORDER_CONFIRM_TYPE_URL => OrderValidation::confirm_type(body,),
            urls::REFUND_URL => RefundValidation::create(body,),
            urls::PLAN_CREATE_URL => PlanValidation::create(body,),
            urls::SUBSCRIPTION_CREATE_URL => SubscriptionValidation::create(body,),
            _ => Err(CustomException::new("Invalid URL",),),
        };

        result
            .map(|_| ("Validation succeeded".to_string(), 200,),)
            .map_err(|e| (e.to_string(), 400,),)
    }

    pub fn list(url: &str, query: &str,) -> Result<(String, u16,), (String, u16,),> {
        let result = match url {
            urls::CARD_URL => CardValidation::list(query,),
            urls::CHARGE_URL => ChargeValidation::list(query,),
            urls::TOKEN_URL => TokenValidation::list(query,),
            urls::CUSTOMER_URL => CustomerValidation::list(query,),
            urls::ORDER_URL => OrderValidation::list(query,),
            urls::REFUND_URL => RefundValidation::list(query,),
            urls::PLAN_URL => PlanValidation::list(query,),
            urls::SUBSCRIPTION_URL => SubscriptionValidation::list(query,),
            _ => Err(CustomException::new("Invalid URL",),),
        };

        result
            .map(|_| ("Validation succeeded".to_string(), 200,),)
            .map_err(|e| (e.to_string(), 400,),)
    }

    pub fn resource_id(url: &str, id: &str,) -> Result<(String, u16,), (String, u16,),> {
        let result = match url {
            urls::CARD_URL => Helpers::validate_start(id, constant::CARD_KEY,),
            urls::CHARGE_URL => Helpers::validate_start(id, constant::CHARGE_KEY,),
            urls::TOKEN_URL => Helpers::validate_start(id, constant::TOKEN_KEY,),
            urls::CUSTOMER_URL => Helpers::validate_start(id, constant::CUSTOMER_KEY,),
            urls::ORDER_URL => Helpers::validate_start(id, constant::ORDER_KEY,),
            urls::REFUND_URL => Helpers::validate_start(id, constant::REFUND_KEY,),
            urls::PLAN_URL => Helpers::validate_start(id, constant::PLAN_KEY,),
            urls::SUBSCRIPTION_URL => Helpers::validate_start(id, constant::SUBSCRIPTION_KEY,),
            _ => Err(CustomException::new("Invalid URL",),),
        };

        result
            .map(|_| ("Validation succeeded".to_string(), 200,),)
            .map_err(|e| (e.to_string(), 400,),)
    }

    pub fn update(url: &str, query: &str,) -> Result<(String, u16,), (String, u16,),> {
        let result = match url {
            urls::CARD_URL => CardValidation::update(query,),
            urls::CHARGE_URL | urls::TOKEN_URL | urls::REFUND_URL => {
                Helpers::validate_metadata_str(query,)
            }
            urls::CUSTOMER_URL => CustomerValidation::update(query,),
            urls::ORDER_URL => OrderValidation::update(query,),
            urls::PLAN_URL => PlanValidation::update(query,),
            urls::SUBSCRIPTION_URL => SubscriptionValidation::update(query,),
            _ => Err(CustomException::new("Invalid URL",),),
        };

        result
            .map(|_| ("Validation succeeded".to_string(), 200,),)
            .map_err(|e| (e.to_string(), 400,),)
    }
}
