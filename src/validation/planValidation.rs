use regex::Regex;
use std::collections::HashMap;
use chrono::{Local, Datelike, NaiveDate};
use serde_json::Value;

use super::{helpers::Helpers, CustomException::CustomException};

pub struct PlanValidation;

impl PlanValidation {
    pub fn create(body: &str) -> Result<(), CustomException> {
        // Parsear el JSON
        let json_data: Value = serde_json::from_str(body)?;

        // Validar campos requeridos
        let required_payload = vec![
            "short_name",
            "description",
            "amount",
            "currency",
            "interval_unit_time",
            "interval_count",
            "initial_cycles",
            "name",
        ];

        for field in &required_payload {
            if !body.contains(field) {
                return Err(CustomException::new(&format!("El campo '{}' es requerido y no está presente", field)));
            }
        }

        // Validar interval_unit_time
        let values_interval_unit_time = [1, 2, 3, 4, 5, 6];
        if let Some(interval_unit_time) = json_data["interval_unit_time"].as_i64() {
            if !values_interval_unit_time.contains(&(interval_unit_time as i32)) {
                return Err(CustomException::new("El campo 'interval_unit_time' tiene un valor inválido. Estos son los únicos valores permitidos: [1, 2, 3, 4, 5, 6]"));
            }
        } else {
            return Err(CustomException::new("El campo 'interval_unit_time' es inválido o está vacío."));
        }

        // Validar interval_count
        let range_interval_count = 0..10000;
        if let Some(interval_count) = json_data["interval_count"].as_i64() {
            if !range_interval_count.contains(&(interval_count as i32)) {
                return Err(CustomException::new("El campo 'interval_count' solo admite valores numéricos en el rango 0 a 9999."));
            }
        } else {
            return Err(CustomException::new("El campo 'interval_count' es inválido o está vacío."));
        }

        // Validar amount
        if let Some(amount) = json_data["amount"].as_i64() {
            // Aquí puedes realizar validaciones adicionales si es necesario
        } else {
            return Err(CustomException::new("El campo 'amount' es inválido o está vacío, debe tener un valor numérico."));
        }

         // Validar name
         let range_name = 5..=50;
         if let Some(name) = json_data["name"].as_str() {
             if !range_name.contains(&(name.len() as i32)) {
                 return Err(CustomException::new("El campo 'name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
             }
         } else {
             return Err(CustomException::new("El campo 'name' es inválido o está vacío."));
         }
 
         // Validar description
         let range_description = 5..=250;
         if let Some(description) = json_data["description"].as_str() {
             if !range_description.contains(&(description.len() as i32)) {
                 return Err(CustomException::new("El campo 'description' es inválido o está vacío. El valor debe tener un rango de 5 a 250 caracteres."));
             }
         } else {
             return Err(CustomException::new("El campo 'description' es inválido o está vacío."));
         }
 
         // Validar short_name
         let range_short_name = 5..=50;
         if let Some(short_name) = json_data["short_name"].as_str() {
             if !range_short_name.contains(&(short_name.len() as i32)) {
                 return Err(CustomException::new("El campo 'short_name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
             }
         } else {
             return Err(CustomException::new("El campo 'short_name' es inválido o está vacío."));
         }

        if let Some(initial_cycles) = json_data["initial_cycles"].as_object() {
            // Llamar a la función de validación de initial_cycles desde Helpers
            Helpers::validate_initial_cycles_parameters(initial_cycles)?;
            
            // Obtener referencias a los valores relevantes para llamar a validate_initial_cycles_parameters
            let has_initial_charge = initial_cycles["has_initial_charge"].as_bool().ok_or_else(|| CustomException::new("El campo 'initial_cycles.has_initial_charge' es inválido o está vacío."))?;
            let currency = json_data["currency"].as_str().ok_or_else(|| CustomException::new("El campo 'currency' es inválido o está vacío."))?;
            print!("Valida amount");
            let amount = json_data["amount"].as_i64().ok_or_else(|| CustomException::new("El campo 'amount' es inválido o está vacío."))? as i32;
                    


            let initial_cycles_amount = initial_cycles["amount"].as_i64().ok_or_else(|| CustomException::new("El campo 'initial_cycles.amount' es inválido o está vacío."))? as i32;
            let count = initial_cycles["count"].as_i64().ok_or_else(|| CustomException::new("El campo 'initial_cycles.count' es inválido o está vacío."))? as i32;
                    
            // Llamar a la función de validación de initial_cycles desde Helpers
            if let Err(err) = Helpers::validate_initial_cycles(has_initial_charge, currency, amount, initial_cycles_amount, count) {
                return Err(err);
            }
            
        } else {
            return Err(CustomException::new("El campo 'initial_cycles' es inválido o está vacío."));
        }

        if body.contains("image") {
            // Verificar si "image" está presente y es una cadena
            if let Some(image_value) = json_data["image"].as_str() {
                // Llamada a la función de validación de la imagen utilizando el valor de cadena
                match Helpers::validate_image(image_value) {
                    Ok(()) => {
                        // La validación de la imagen fue exitosa, continúa con el código aquí
                    }
                    Err(err) => {
                        // La validación de la imagen falló, maneja el error aquí
                        eprintln!("Error de validación de imagen: {}", err);
                    }
                }
            } else {
                // El campo "image" no es una cadena, manejar según sea necesario
                eprintln!("El campo 'image' no es una cadena.");
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

    pub fn update(body: &str) -> Result<(), CustomException> {
        let json_data: Value = serde_json::from_str(body)?;
        if body.contains("name") {
            let range_name = 5..=50;
            if let Some(name) = json_data["name"].as_str() {
                if !range_name.contains(&(name.len() as i32)) {
                    return Err(CustomException::new("El campo 'name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
                }
            } else {
                return Err(CustomException::new("El campo 'name' es inválido o está vacío."));
            }
        }
        if body.contains("description") {
        // Validar description
            let range_description = 5..=250;
            if let Some(description) = json_data["description"].as_str() {
                if !range_description.contains(&(description.len() as i32)) {
                    return Err(CustomException::new("El campo 'description' es inválido o está vacío. El valor debe tener un rango de 5 a 250 caracteres."));
                }
            } else {
                return Err(CustomException::new("El campo 'description' es inválido o está vacío."));
            }
        }

        if body.contains("short_name") {
            // Validar short_name
            let range_short_name = 5..=50;
            if let Some(short_name) = json_data["short_name"].as_str() {
                if !range_short_name.contains(&(short_name.len() as i32)) {
                    return Err(CustomException::new("El campo 'short_name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
                }
            } else {
                return Err(CustomException::new("El campo 'short_name' es inválido o está vacío."));
            }
        }

        if body.contains("image") {
            // Verificar si "image" está presente y es una cadena
            if let Some(image_value) = json_data["image"].as_str() {
                // Llamada a la función de validación de la imagen utilizando el valor de cadena
                match Helpers::validate_image(image_value) {
                    Ok(()) => {
                        // La validación de la imagen fue exitosa, continúa con el código aquí
                    }
                    Err(err) => {
                        // La validación de la imagen falló, maneja el error aquí
                        eprintln!("Error de validación de imagen: {}", err);
                    }
                }
            } else {
                // El campo "image" no es una cadena, manejar según sea necesario
                eprintln!("El campo 'image' no es una cadena.");
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
     // Validate parameters status
     if let Some(status) = json_data.get("status") {
        let values_status = [1, 2];
        if !status.is_i64() || !values_status.contains(&status.as_i64().unwrap()) {
            return Err(CustomException::new("El filtro 'status' tiene un valor inválido o está vacío. Estos son los únicos valores permitidos: 1, 2."));
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

    // Validate parameters max_amount
    if let Some(max_amount) = json_data.get("max_amount") {
        let range_max_amount = 300..=500000;
        if !max_amount.is_i64() || !range_max_amount.contains(&max_amount.as_i64().unwrap()) {
            return Err(CustomException::new("El filtro 'max_amount' admite valores en el rango 300 a 500000."));
        }
    }

    // Validate parameters min_amount
    if let Some(min_amount) = json_data.get("min_amount") {
        let range_min_amount = 300..=500000;
        if !min_amount.is_i64() || !range_min_amount.contains(&min_amount.as_i64().unwrap()) {
            return Err(CustomException::new("El filtro 'min_amount' admite valores en el rango 300 a 500000."));
        }
    }
        Ok(())
    }
}