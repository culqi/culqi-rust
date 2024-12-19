use super::cardValidation::CardValidation;
use super::chargeValidation::ChargeValidation;
use super::customerValidation::CustomerValidation;
use super::helpers::Helpers;
use super::orderValidation::OrderValidation;
use super::planValidation::PlanValidation;
use super::refundValidation::RefundValidation;
use super::subscriptionValidation::SubscriptionValidation;
use super::tokenValidation::TokenValidation;
use crate::utils::urls::{ORDER_URL, TOKEN_URL, TOKEN_YAPE_URL};
use crate::utils::CustomException::CustomException;

use anyhow::Result;
use hyper::StatusCode;
use warp::reply::{self, Response};
use warp::Reply;
pub struct ValidateIfAction;

impl ValidateIfAction {
    pub fn validate_class(action: &str, body: &str) -> Result<impl Reply, warp::Rejection> {
        if action == TOKEN_URL {
            match TokenValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action == "charges" {
            match ChargeValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action == "cards" {
            match CardValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action == "customers" {
            match CustomerValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action.contains("plans") {
            match PlanValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    // Aquí puedes retornar un CustomException en lugar de una cadena
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action == "refunds" {
            match RefundValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action.contains("subscriptions") {
            match SubscriptionValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }
        if action == ORDER_URL {
            match OrderValidation::create(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));
                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        if action == TOKEN_YAPE_URL {
            return Ok(reply::with_status(
                reply::html("Validation succeeded"),
                StatusCode::OK,
            ));
        }

        return Ok(reply::with_status(
            reply::html("Invalid action"),
            StatusCode::BAD_REQUEST,
        ));
    }

    pub fn validate_update_class(action: &str, body: &str) -> Result<impl Reply, warp::Rejection> {
        if action.contains("plans") {
            match PlanValidation::update(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::update(body) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }

        let custom_error = CustomException::new(&"Invalid action specified".to_string());
        return Err(warp::reject::custom(custom_error)); // Rechazo con CustomException
    }

    pub fn validate_all_class(action: &str, query: &str) -> Result<impl Reply, warp::Rejection> {
        if action.contains("plans") {
            match PlanValidation::list(query) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }
        if action.contains("subscriptions") {
            match SubscriptionValidation::list(query) {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }
        return Ok(reply::with_status(
            reply::html("Validation succeeded"),
            StatusCode::OK,
        ));
    }

    pub fn validate_id_class(action: &str, query: &str) -> Result<impl Reply, warp::Rejection> {
        if action.contains("plans") {
            match Helpers::validate_id(query, "pln") {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }
        if action.contains("subscriptions") {
            match Helpers::validate_id(query, "sxn") {
                Ok(_) => {
                    return Ok(reply::with_status(
                        reply::html("Validation succeeded"),
                        StatusCode::OK,
                    ));                }
                Err(e) => {
                    return Err(warp::reject::custom(e)); // Rechazo con el CustomException
                }
            }
        }
        return Ok(reply::with_status(
            reply::html("Validation succeeded"),
            StatusCode::OK,
        ));    }
}
