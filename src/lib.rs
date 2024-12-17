#![crate_name = "BrandoCulqi"]
#![allow(non_snake_case)]

pub mod utils;
pub mod client;
pub mod culqi {
    pub mod order;
}

extern crate hyper;

use crate::utils::encrypt::encrypt;
use crate::utils::headers::CustomHeaders;
use crate::utils::urls::{BASE_URL, SECURE_URL};

use reqwest::Client;
use std::error::Error;
mod validation;
use anyhow::Result;
use validation::validate_if_action::ValidateIfAction;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//--------------funciones- genericas---------------
pub async fn create(body: &str, action: &str, pk: &str, sk: &str) -> Result<(String, u16)> {
    let skey: &str = sk;
    let pkey: &str = pk;

    let key: &str;
    let url: String;

    ValidateIfAction::validate_class(action, body)?;
    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        if action == "plans" || action == "subscriptions" {
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        } else {
            url = BASE_URL.to_owned() + action;
        }
    }

    let client = Client::new();
    let additional_headers = CustomHeaders::get_headers(key);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", key))
        .headers(additional_headers)
        .body(body.to_owned()) // Clonar el contenido del body a un String propio
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn create_with_custom_headers(
    body: &str,
    action: &str,
    pk: &str,
    sk: &str,
    custom_headers: &str,
) -> Result<(String, u16)> {
    let skey: &str = sk;
    let pkey: &str = pk;

    let key: &str;
    let url: String;

    ValidateIfAction::validate_class(action, body)?;
    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        if action == "plans" || action == "subscriptions" {
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        } else {
            url = BASE_URL.to_owned() + action;
        }
    }

    let client = Client::new();
    let additional_headers = CustomHeaders::get_custom_headers(key, custom_headers);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", key))
        .headers(additional_headers)
        .body(body.to_owned())
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn update(
    body: &str,
    action: &str,
    query: &str,
    pk: &str,
    sk: &str,
) -> Result<(String, u16)> {
    let skey: &str = sk;
    let pkey: &str = pk;

    let key: &str;
    let url: String;

    ValidateIfAction::validate_id_class(action, query)?;
    ValidateIfAction::validate_update_class(action, body)?;
    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        if action == "plans" || action == "subscriptions" {
            url = BASE_URL.to_owned() + "recurrent/" + action + "/" + query;
        } else {
            url = BASE_URL.to_owned() + action;
        }
    }

    let client = Client::new();
    let additional_headers = CustomHeaders::get_headers(key);

    let response = client
        .patch(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", key))
        .headers(additional_headers)
        .body(body.to_owned()) // Clonar el contenido del body a un String propio
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

pub async fn create_encrypt(
    body: &str,
    action: &str,
    pkey: &str,
    skey: &str,
    rsa_pkey: &str,
    rsa_pid: &str,
) -> Result<(String, u16), Box<dyn Error>> {
    ValidateIfAction::validate_class(action, body)?;

    let body_encrypt = encrypt(body, rsa_pkey, true)?;

    let key: &str;
    let url: String;

    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        if action == "plans" || action == "subscriptions" {
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        } else {
            url = BASE_URL.to_owned() + action;
        }
    }

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_pid).parse().unwrap());

    let additional_headers = CustomHeaders::get_headers(key);

    // Añade aquí cualquier otro encabezado que necesites

    let response = client
        .post(&url)
        .headers(headers)
        .headers(additional_headers)
        .body(serde_json::to_string(&body_encrypt).unwrap())
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn create_encrypt_with_custom_headers(
    body: &str,
    action: &str,
    pkey: &str,
    skey: &str,
    rsa_pkey: &str,
    rsa_pid: &str,
    custom_headers: &str,
) -> Result<(String, u16), Box<dyn Error>> {
    ValidateIfAction::validate_class(action, body)?;

    let body_encrypt = encrypt(body, rsa_pkey, true)?;

    let key: &str;
    let url: String;

    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        if action == "plans" || action == "subscriptions" {
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        } else {
            url = BASE_URL.to_owned() + action;
        }
    }

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_pid).parse().unwrap());

    let additional_headers = CustomHeaders::get_custom_headers(key, custom_headers);

    // Añade aquí cualquier otro encabezado que necesites

    let response = client
        .post(&url)
        .headers(headers)
        .headers(additional_headers)
        .body(serde_json::to_string(&body_encrypt).unwrap())
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn get(action: &str, query: &str, skey: &str) -> Result<(String, u16)> {
    let key: &str;
    let url: String;
    ValidateIfAction::validate_id_class(action, query)?;
    key = skey;
    if action == "plans" || action == "subscriptions" {
        url = BASE_URL.to_owned() + "recurrent/" + action + "/" + query;
    } else {
        url = BASE_URL.to_owned() + action + "/" + query;
    }

    let client = reqwest::Client::new();

    let additional_headers = CustomHeaders::get_headers(key);

    let response = client
        .get(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .headers(additional_headers)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn all(action: &str, query: &str, skey: &str) -> Result<(String, u16)> {
    let key: &str;
    let url: String;
    ValidateIfAction::validate_all_class(action, query)?;

    let json_value: Value = serde_json::from_str(query).unwrap();
    let query_string = json_value.as_object().map_or(String::new(), |obj| {
        obj.iter()
            .map(|(key, value)| {
                // Verificar si el valor es una cadena antes de incluirlo en la cadena de consulta
                if let Some(string_value) = value.as_str() {
                    format!("{}={}", key, string_value)
                } else {
                    // Si no es una cadena, convertir a texto sin comillas
                    format!("{}={}", key, value.to_string().trim_matches('"'))
                }
            })
            .collect::<Vec<_>>()
            .join("&")
    });
    key = skey;
    if action == "plans" || action == "subscriptions" {
        url = BASE_URL.to_owned() + "recurrent/" + action + "?" + &query_string;
        print!("{}", url)
    } else {
        url = BASE_URL.to_owned() + action + "?" + &query_string;
    }

    let client = reqwest::Client::new();

    let additional_headers = CustomHeaders::get_headers(key);

    let response = client
        .get(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .headers(additional_headers)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn delete(action: &str, query: &str, skey: &str) -> Result<(String, u16)> {
    let key: &str;
    let url: String;
    ValidateIfAction::validate_id_class(action, query)?;
    key = skey; // Asegúrate de definir skey
    if action == "plans" || action == "subscriptions" {
        url = BASE_URL.to_owned() + "recurrent/" + action + "/" + query;
    } else {
        url = BASE_URL.to_owned() + action + "/" + query;
    }

    let client = reqwest::Client::new();
    let additional_headers = CustomHeaders::get_headers(key);

    let response = client
        .delete(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .headers(additional_headers)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}
