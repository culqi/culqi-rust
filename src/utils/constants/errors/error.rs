pub const ERROR_ENCRYPT: &str = "Error al encriptar:";
pub const ERROR_RSA: &str = "Es requerida una clave RSA";
pub const GENERIC_ERROR: &str = "Se presento un error en la solicitud.";
pub const REQUEST_ERROR: &str = "Error en el cuerpo de la peticion.";

pub const ERROR_FILTER_DATE: &str = "Invalid value. date_from must be less than date_to";
pub const INVALID_EMAIL: &str = "Invalid email";
pub const INVALID_CARD_NUMBER: &str = "Invalid card_number";
pub const INVALID_CVV: &str = "Invalid CVV";
pub const INVALID_MONTH: &str = "Invalid month";
pub const INVALID_YEAR: &str = "Invalid year";
pub const CARD_EXPIRED: &str = "Card has expired";
pub const INVALID_EXPIRATION_EXPIRED: &str = "Invalid expiration date format.";
pub const INVALID_EXPIRATION_FUTURE_DATE: &str = "expiration_date must be a future date.";
pub const INVALID_CURRENCY_CODE: &str = "Currency code must be either \"PEN\" or \"USD\".";

pub fn validate_key_id(key: &str,) -> String {
    format!("{} not found or is not a string ", key)
}

pub fn validate_string_start(start: &str,) -> String {
    format!(
        "Incorrect format. The format must start with {}_test_ or {}_live_",
        start, start
    )
}

pub fn validate_allow_values<T: std::fmt::Debug,>(allowed_values: &[T],) -> String {
    format!("Invalid value. It must be one of {:?}", allowed_values)
}

pub fn validate_type_value(value: &str, types: &str,) -> String {
    format!("Error -> parameter {} is {}.", value, types)
}

pub fn not_present_parameter(value: &str,) -> String {
    format!("El campo '{}' es requerido o es invalido.", value,)
}
