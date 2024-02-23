use regex::Regex;
use std::collections::HashMap;
use chrono::{Local, Datelike, NaiveDate};
use serde_json::Value;

use super::{helpers::Helpers, CustomException::CustomException};

pub struct SubscriptionValidation;

impl SubscriptionValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        let json_data: Value = serde_json::from_str(body)?;
        let required_payload = vec![
            "card_id",
            "plan_id",
            "tyc"
        ];
        for field in &required_payload {
            if !body.contains(field) {
                return Err(CustomException::new(&format!("El campo '{}' es requerido.", field)));
            }
        }

        if let Some(card_id) = json_data.get("card_id") {
            if let Some(card_id_str) = card_id.as_str() {
                if card_id_str.len() != 25 {
                    return Err(CustomException::new("El campo 'card_id' es inválido. La longitud debe ser de 25."));
                }
            } else {
                return Err(CustomException::new("El campo 'card_id' es inválido. La longitud debe ser de 25."));
            }
        }
        if let Some(source_id) = json_data.get("card_id").and_then(Value::as_str) {
            Helpers::validate_string_start(source_id, "crd")?;
        } else {
            return Err(CustomException::new("card_id not found or is not a string"));
        }

        if let Some(plan_id) = json_data.get("plan_id") {
            if let Some(plan_id_str) = plan_id.as_str() {
                if plan_id_str.len() != 25 {
                    return Err(CustomException::new("El campo 'plan_id' es inválido. La longitud debe ser de 25."));
                }
            } else {
                return Err(CustomException::new("El campo 'plan_id' es inválido. La longitud debe ser de 25."));
            }
        }      
       
        if let Some(source_id) = json_data.get("plan_id").and_then(Value::as_str) {
                Helpers::validate_string_start(source_id, "pln")?;
        } else {
            return Err(CustomException::new("plan_id not found or is not a string"));
        }

        if body.contains("metadata") {
            match Helpers::validate_metadata(&json_data["metadata"]) {
                Ok(()) => {
                    // La validación fue exitosa, continúa con el código aquí
                }
                Err(err) => {
                    // Manejar el error, por ejemplo, imprimir el mensaje de error
                    eprintln!("Error al validar metadata: {}", err);
                    // También puedes devolver el error, lanzarlo, etc., según tus necesidades
                    return Err(err);
                }
            }
        }

        if let Some(tyc_value) = json_data.get("tyc") {
            if !tyc_value.is_boolean() {
                return Err(CustomException::new("El campo 'tyc' es inválido o está vacío. El valor debe ser un booleano."));
            }
        }

        Ok(())
    }

    pub fn update(body: &str) -> Result<(), CustomException> {
        let json_data: Value = serde_json::from_str(body)?;
        let required_payload = vec![
            "card_id"
        ];
        for field in &required_payload {
            if !body.contains(field) {
                return Err(CustomException::new(&format!("El campo '{}' es requerido.", field)));            }
        }

        if let Some(card_id) = json_data.get("card_id") {
            if let Some(card_id_str) = card_id.as_str() {
                if card_id_str.len() != 25 {
                    return Err(CustomException::new("El campo 'card_id' es inválido. La longitud debe ser de 25."));
                }
            } else {
                return Err(CustomException::new("El campo 'card_id' es inválido. La longitud debe ser de 25."));
            }
        }

        if body.contains("metadata") {
            match Helpers::validate_metadata(&json_data["metadata"]) {
                Ok(()) => {
                    // La validación fue exitosa, continúa con el código aquí
                }
                Err(err) => {
                    // Manejar el error, por ejemplo, imprimir el mensaje de error
                    eprintln!("Error al validar metadata: {}", err);
                    // También puedes devolver el error, lanzarlo, etc., según tus necesidades
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    pub fn list(query: &str) -> Result<(), CustomException> {
        let json_data: Value = serde_json::from_str(query)?;

        if let Some(plan_id) = json_data.get("plan_id") {
            if !plan_id.is_string() || !(plan_id.as_str().unwrap().len() == 25) {
                return Err(CustomException::new("El campo 'plan_id' es inválido. La longitud debe ser de 25."));
            }
        }

        // Validate parameters status
        if let Some(status) = json_data.get("status") {
            let values_status = [1, 2, 3, 4, 5, 6, 8];
            if !status.is_i64() || !values_status.contains(&status.as_i64().unwrap()) {
                return Err(CustomException::new("El filtro 'status' tiene un valor inválido o está vacío. Estos son los únicos valores permitidos: 1, 2, 3, 4, 5, 6, 8."));
            }
        }

         // Validate parameters creation_date_from
        if let Some(creation_date_from) = json_data.get("creation_date_from") {
            if !creation_date_from.is_string() || !(creation_date_from.as_str().unwrap().len() == 10 || creation_date_from.as_str().unwrap().len() == 13) {
                return Err(CustomException::new("El campo 'creation_date_from' debe tener una longitud de 10 o 13 caracteres."));
            }
        }

        // Validate parameters creation_date_to
        if let Some(creation_date_to) = json_data.get("creation_date_to") {
            if !creation_date_to.is_string() || !(creation_date_to.as_str().unwrap().len() == 10 || creation_date_to.as_str().unwrap().len() == 13) {
                return Err(CustomException::new("El campo 'creation_date_to' debe tener una longitud de 10 o 13 caracteres."));
            }
        }

        // Validate parameters before
        if let Some(before) = json_data.get("before") {
            if !before.is_string() || before.as_str().unwrap().len() != 25 {
                return Err(CustomException::new("El campo 'before' es inválido. La longitud debe ser de 25 caracteres."));
            }
        }

        // Validate parameters after
        if let Some(after) = json_data.get("after") {
            if !after.is_string() || after.as_str().unwrap().len() != 25 {
                return Err(CustomException::new("El campo 'after' es inválido. La longitud debe ser de 25 caracteres."));
            }
        }

        // Validate parameters limit
        if let Some(limit) = json_data.get("limit") {
            let range_limit = 1..=100;
            if !limit.is_i64() || !range_limit.contains(&limit.as_i64().unwrap()) {
                return Err(CustomException::new("El filtro 'limit' admite valores en el rango 1 a 100."));
            }
        }

        Ok(())
    }
}