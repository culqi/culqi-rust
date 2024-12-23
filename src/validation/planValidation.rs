use serde_json::Value;

use super::helpers::Helpers;
use crate::utils::CustomException::CustomException;

pub struct PlanValidation;

impl PlanValidation {
    pub fn create(body: &str,) -> Result<(), CustomException,> {
        let json_data: Value = serde_json::from_str(body,)?;

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
            if !body.contains(field,) {
                return Err(CustomException::new(&format!(
                    "El campo '{}' es requerido y no está presente",
                    field
                ),),);
            }
        }

        let values_interval_unit_time = [1, 2, 3, 4, 5, 6,];
        if let Some(interval_unit_time,) = json_data["interval_unit_time"].as_i64() {
            if !values_interval_unit_time.contains(&(interval_unit_time as i32),) {
                return Err(CustomException::new(
            "El campo 'interval_unit_time' tiene un valor inválido. Estos son los únicos valores permitidos: [1, 2, 3, 4, 5, 6]"
                ));
            }
        } else {
            println!("entro qui");
            return Err(CustomException::new(
                "El campo 'interval_unit_time' es inválido o está vacío.",
            ),);
        }

        let range_interval_count = 0..10000;
        if let Some(interval_count,) = json_data["interval_count"].as_i64() {
            if !range_interval_count.contains(&(interval_count as i32),) {
                return Err(CustomException::new(
                    "El campo 'interval_count' solo admite valores numéricos en el rango 0 a 9999.",
                ),);
            }
        } else {
            return Err(CustomException::new(
                "El campo 'interval_count' es inválido o está vacío.",
            ),);
        }

        if let Some(_amount,) = json_data["amount"].as_i64() {
        } else {
            return Err(CustomException::new(
                "El campo 'amount' es inválido o está vacío, debe tener un valor numérico.",
            ),);
        }

        let range_name = 5..=50;
        if let Some(name,) = json_data["name"].as_str() {
            if !range_name.contains(&(name.len() as i32),) {
                return Err(CustomException::new("El campo 'name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
            }
        } else {
            return Err(CustomException::new(
                "El campo 'name' es inválido o está vacío.",
            ),);
        }

        let range_description = 5..=250;
        if let Some(description,) = json_data["description"].as_str() {
            if !range_description.contains(&(description.len() as i32),) {
                return Err(CustomException::new("El campo 'description' es inválido o está vacío. El valor debe tener un rango de 5 a 250 caracteres."));
            }
        } else {
            return Err(CustomException::new(
                "El campo 'description' es inválido o está vacío.",
            ),);
        }

        let range_short_name = 5..=50;
        if let Some(short_name,) = json_data["short_name"].as_str() {
            if !range_short_name.contains(&(short_name.len() as i32),) {
                return Err(CustomException::new("El campo 'short_name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
            }
        } else {
            return Err(CustomException::new(
                "El campo 'short_name' es inválido o está vacío.",
            ),);
        }

        if let Some(initial_cycles,) = json_data["initial_cycles"].as_object() {
            Helpers::validate_initial_cycles_parameters(initial_cycles,)?;

            let has_initial_charge =
                initial_cycles["has_initial_charge"].as_bool().ok_or_else(|| {
                    CustomException::new(
                        "El campo 'initial_cycles.has_initial_charge' es inválido o está vacío.",
                    )
                },)?;
            let currency = json_data["currency"].as_str().ok_or_else(|| {
                CustomException::new("El campo 'currency' es inválido o está vacío.",)
            },)?;
            let count = initial_cycles["count"].as_i64().ok_or_else(|| {
                CustomException::new("El campo 'initial_cycles.count' es inválido o está vacío.",)
            },)? as i32;

            if let Err(err,) =
                Helpers::validate_initial_cycles(has_initial_charge, currency, count,)
            {
                return Err(err,);
            }
        } else {
            return Err(CustomException::new(
                "El campo 'initial_cycles' es inválido o está vacío.",
            ),);
        }

        if body.contains("image",) {
            if let Some(image_value,) = json_data["image"].as_str() {
                match Helpers::validate_image(image_value,) {
                    Ok((),) => {}
                    Err(err,) => {
                        eprintln!("Error de validación de imagen: {}", err);
                    }
                }
            } else {
                eprintln!("El campo 'image' no es una cadena.");
            }
        }

        if body.contains("metadata",) {
            match Helpers::validate_metadata(&json_data["metadata"],) {
                Ok((),) => {}
                Err(err,) => {
                    eprintln!("Error al validar metadata: {}", err);
                    return Err(err,);
                }
            }
        }

        Ok((),)
    }

    pub fn update(body: &str,) -> Result<(), CustomException,> {
        let json_data: Value = serde_json::from_str(body,)?;
        if body.contains("name",) {
            let range_name = 5..=50;
            if let Some(name,) = json_data["name"].as_str() {
                if !range_name.contains(&(name.len() as i32),) {
                    return Err(CustomException::new("El campo 'name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
                }
            } else {
                return Err(CustomException::new(
                    "El campo 'name' es inválido o está vacío.",
                ),);
            }
        }
        if body.contains("description",) {
            let range_description = 5..=250;
            if let Some(description,) = json_data["description"].as_str() {
                if !range_description.contains(&(description.len() as i32),) {
                    return Err(CustomException::new("El campo 'description' es inválido o está vacío. El valor debe tener un rango de 5 a 250 caracteres."));
                }
            } else {
                return Err(CustomException::new(
                    "El campo 'description' es inválido o está vacío.",
                ),);
            }
        }

        if body.contains("short_name",) {
            let range_short_name = 5..=50;
            if let Some(short_name,) = json_data["short_name"].as_str() {
                if !range_short_name.contains(&(short_name.len() as i32),) {
                    return Err(CustomException::new("El campo 'short_name' es inválido o está vacío. El valor debe tener un rango de 5 a 50 caracteres."));
                }
            } else {
                return Err(CustomException::new(
                    "El campo 'short_name' es inválido o está vacío.",
                ),);
            }
        }

        if body.contains("image",) {
            if let Some(image_value,) = json_data["image"].as_str() {
                match Helpers::validate_image(image_value,) {
                    Ok((),) => {}
                    Err(err,) => {
                        eprintln!("Error de validación de imagen: {}", err);
                    }
                }
            } else {
                eprintln!("El campo 'image' no es una cadena.");
            }
        }

        if body.contains("metadata",) {
            match Helpers::validate_metadata(&json_data["metadata"],) {
                Ok((),) => {}
                Err(err,) => {
                    eprintln!("Error al validar metadata: {}", err);
                    return Err(err,);
                }
            }
        }

        Ok((),)
    }

    pub fn list(query: &str,) -> Result<(), CustomException,> {
        let json_data: Value = serde_json::from_str(query,)?;
        if let Some(status,) = json_data.get("status",) {
            let values_status = [1, 2,];
            if !status.is_i64() || !values_status.contains(&status.as_i64().unwrap(),) {
                return Err(CustomException::new("El filtro 'status' tiene un valor inválido o está vacío. Estos son los únicos valores permitidos: 1, 2."));
            }
        }

        if let Some(creation_date_from,) = json_data.get("creation_date_from",) {
            if !creation_date_from.is_string()
                || !(creation_date_from.as_str().unwrap().len() == 10
                    || creation_date_from.as_str().unwrap().len() == 13)
            {
                return Err(CustomException::new(
                    "El campo 'creation_date_from' debe tener una longitud de 10 o 13 caracteres.",
                ),);
            }
        }

        if let Some(creation_date_to,) = json_data.get("creation_date_to",) {
            if !creation_date_to.is_string()
                || !(creation_date_to.as_str().unwrap().len() == 10
                    || creation_date_to.as_str().unwrap().len() == 13)
            {
                return Err(CustomException::new(
                    "El campo 'creation_date_to' debe tener una longitud de 10 o 13 caracteres.",
                ),);
            }
        }

        if let Some(before,) = json_data.get("before",) {
            if !before.is_string() || before.as_str().unwrap().len() != 25 {
                return Err(CustomException::new(
                    "El campo 'before' es inválido. La longitud debe ser de 25 caracteres.",
                ),);
            }
        }

        if let Some(after,) = json_data.get("after",) {
            if !after.is_string() || after.as_str().unwrap().len() != 25 {
                return Err(CustomException::new(
                    "El campo 'after' es inválido. La longitud debe ser de 25 caracteres.",
                ),);
            }
        }

        if let Some(limit,) = json_data.get("limit",) {
            let range_limit = 1..=100;
            if !limit.is_i64() || !range_limit.contains(&limit.as_i64().unwrap(),) {
                return Err(CustomException::new(
                    "El filtro 'limit' admite valores en el rango 1 a 100.",
                ),);
            }
        }

        if let Some(max_amount,) = json_data.get("max_amount",) {
            if !max_amount.is_i64() {
                return Err(CustomException::new(
                    "El filtro 'max_amount' es invalido, debe tener un valor numérico entero.",
                ),);
            }
        }

        if let Some(min_amount,) = json_data.get("min_amount",) {
            if !min_amount.is_i64() {
                return Err(CustomException::new(
                    "El filtro 'min_amount' es invalido, debe tener un valor numérico entero.",
                ),);
            }
        }
        Ok((),)
    }
}
