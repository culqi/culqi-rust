use crate::utils::CustomException::CustomException;
extern crate regex;
use std::collections::{HashMap, HashSet};

use chrono::{Utc, TimeZone};
use regex::Regex;
use serde_json::Value;

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
        let exp_date = Utc.timestamp_opt(expiration_date, 0).single();
        exp_date.map_or(false, |d| d > Utc::now())
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

    pub fn validate_initial_cycles_parameters(initial_cycles: &serde_json::Map<String, Value>) -> Result<(), CustomException> {
        let parameters_initial_cycles = vec![
            "count",
            "has_initial_charge",
            "amount",
            "interval_unit_time",
        ];
    
        // Convertir el serde_json::Map a un HashMap<&str, &Value>
        let initial_cycles: HashMap<&str, &Value> = initial_cycles.iter().map(|(k, v)| (k.as_str(), v)).collect();
    
        for campo in &parameters_initial_cycles {
            if !initial_cycles.contains_key(campo) {
                return Err(CustomException::new(&format!("El campo obligatorio '{}' no está presente en 'initial_cycles'.", campo)));
            }
        }
    
        if let Some(_count) = initial_cycles.get("count").and_then(|v| v.as_i64()) {
            // Se ha obtenido un valor válido de tipo i64
        } else {
            return Err(CustomException::new("El campo 'initial_cycles.count' es inválido o está vacío."));
        }
    
        if let Some(_has_initial_charge) = initial_cycles.get("has_initial_charge").and_then(|v| v.as_bool()) {
            // Se ha obtenido un valor válido de tipo bool
        } else {
            return Err(CustomException::new("El campo 'initial_cycles.has_initial_charge' es inválido o está vacío."));
        }
    
        if let Some(_amount) = initial_cycles.get("amount").and_then(|v| v.as_i64()) {
            // Se ha obtenido un valor válido de tipo i64
        } else {
            return Err(CustomException::new("El campo 'initial_cycles.amount' es inválido o está vacío."));
        }
    
        let values_interval_unit_time = [1, 2, 3, 4, 5, 6];
        if let Some(interval_unit_time) = initial_cycles.get("interval_unit_time").and_then(|v| v.as_i64()) {
            if !values_interval_unit_time.contains(&(interval_unit_time as i32)) {
                return Err(CustomException::new("El campo 'initial_cycles.interval_unit_time' tiene un valor inválido o está vacío. Estos son los únicos valores permitidos: [1, 2, 3, 4, 5, 6]"));
            }
        } else {
            return Err(CustomException::new("El campo 'initial_cycles.interval_unit_time' es inválido o está vacío."));
        }
    
        Ok(())
    }

    pub fn validate_enum_currency(currency: &str) -> Result<(), CustomException> {
        let allowed_values = ["PEN", "USD"];
    
        if allowed_values.contains(&currency) {
            Ok(())
        } else {
            Err(CustomException::new(&format!("El campo 'currency' es inválido o está vacío, el código de la moneda en tres letras (Formato ISO 4217). Culqi actualmente soporta las siguientes monedas: {:?}", allowed_values)))
        }
    }

    pub fn validate_initial_cycles(has_initial_charge: bool, currency: &str, count: i32) -> Result<(), CustomException> {
        if has_initial_charge {
            Self::validate_enum_currency(currency)?;
        
            if !(1 <= count && count <= 9999) {
                return Err(CustomException::new("El campo 'initial_cycles.count' solo admite valores numéricos en el rango 1 a 9999."));
            }
    
        } else {
            if !(0 <= count && count <= 9999) {
                return Err(CustomException::new("El campo 'initial_cycles.count' solo admite valores numéricos en el rango 0 a 9999."));
            }

        }
    
        Ok(())
    }

    pub fn validate_image(image: &str) -> Result<(), CustomException> {
        let regex_image = r"^(http:\/\/www\.|https:\/\/www\.|http:\/\/|https:\/\/)?[a-zA-Z0-9]+([-.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,5}(:[0-9]{1,5})?(\/.*)?$";
        let regex = Regex::new(&regex_image).map_err(|_| CustomException::new("Error en la expresión regular para validar la imagen."))?;
        if !(image.len() >= 5 && image.len() <= 250 && regex.is_match(image)) {
            return Err(CustomException::new("El campo 'image' es inválido. Debe ser una cadena y una URL válida."));
        }
    
        Ok(())
    }
    
    pub fn validate_metadata(metadata: &Value) -> Result<(), CustomException> {
        if let Some(err) = Self::validate_key_and_value_length(metadata) {
            return Err(err);
        }
    
        Ok(())
    }
    
    pub fn validate_key_and_value_length(obj_metadata: &Value) -> Option<CustomException> {
        let max_key_length = 30;
        let max_value_length = 200;
    
        if let Some(obj_metadata) = obj_metadata.as_object() {
            for (key, value) in obj_metadata {
                let key_str = key.to_string();
                let value_str = value.to_string();
    
                if !(1 <= key_str.len() && key_str.len() <= max_key_length) ||
                   !(1 <= value_str.len() && value_str.len() <= max_value_length) {
                    let error_message = format!(
                        "El objeto 'metadata' es inválido, límite key (1 - {}), value (1 - {}).",
                        max_key_length, max_value_length
                    );
                    return Some(CustomException::new(&error_message));
                }
            }
    
            None
        } else {
            Some(CustomException::new("El objeto 'metadata' no es un diccionario."))
        }
    }

    pub fn validate_id(id: &str, val: &str) -> Result<(), CustomException> {
        if id.is_empty() || id.len() < 25 {
            return Err(CustomException::new("El campo 'id' es inválido. La longitud debe ser de 25 caracteres."));
        }

        Self::validate_string_start(id, val)?;
    
        Ok(())
    }

}
