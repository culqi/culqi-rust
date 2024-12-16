use super::helpers::Helpers;
use crate::utils::CustomException::CustomException;
use serde_json::Value;

pub struct RefundValidation;

impl RefundValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let parsed: Value = serde_json::from_str(body)?;
        let charge_id =
            parsed
                .get("charge_id")
                .and_then(Value::as_str)
                .ok_or(CustomException::new(
                    "Charge ID not found or is not a string",
                ))?;
        Helpers::validate_string_start(charge_id, "chr")?;

        let allowed_reason_values = ["duplicado", "fraudulento", "solicitud_comprador"];
        let reason = parsed
            .get("reason")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("Reason not found or is not a string"))?;
        Helpers::validate_value(reason, &allowed_reason_values)?;

        let amount_obj = parsed
            .get("amount")
            .and_then(Value::as_str)
            .ok_or(CustomException::new("Amount not found"))?;
        Helpers::validate_amount_value(amount_obj)?;

        Ok(())
    }
}
