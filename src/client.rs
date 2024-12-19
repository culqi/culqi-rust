use std::collections::HashMap;

use crate::utils::encrypt::encrypt;
use crate::utils::CustomException::CustomException;
use crate::{
    utils::urls::{BASE_URL, SECURE_URL, TOKEN_URL},
    validation::validate_if_action::ValidateIfAction,
};
use anyhow::Result;
use hyper::header::HeaderName;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::{from_str, Value};

use warp::http::StatusCode;
use warp::reply::{self, Response};
use warp::Reply;
pub struct Client {
    secret_key: String,
    public_key: String,
    rsa_key: Option<String>,
    client: reqwest::Client,
}

impl Client {
    pub fn config(secret_key: &str, public_key: &str, rsa_key: Option<&str>) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            rsa_key: rsa_key.map(|key| key.to_string()),
            client: reqwest::Client::new(),
        }
    }

    pub fn get_headers(
        &self,
        isToken: bool,
        custom_headers: Option<&str>,
    ) -> Result<HeaderMap, CustomException> {
        let mut headers = HeaderMap::new();

        let token = if isToken {
            &self.public_key
        } else {
            &self.secret_key
        };

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        if let Some(custom_headers_str) = custom_headers {
            match from_str::<HashMap<String, Value>>(custom_headers_str) {
                Ok(custom_headers_map) => {
                    for (header_key, header_value) in custom_headers_map {
                        let header_value_str = match header_value {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => String::new(), // Caso por defecto si el tipo es desconocido
                        };

                        if !header_value_str.is_empty() {
                            if let Ok(header_name) = HeaderName::from_bytes(header_key.as_bytes()) {
                                if let Ok(header_value) = HeaderValue::from_str(&header_value_str) {
                                    headers.insert(header_name, header_value);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    // Si hay un error al deserializar la cadena JSON, lo registramos.
                    return Err(CustomException::new(&format!(
                        "Error al deserializar los encabezados personalizados: {}",
                        e
                    )));
                }
            }
        }

        Ok(headers)
    }

    pub async fn post<T: Serialize>(
        &self,
        path: &str,
        body: &T,
        custom_headers: Option<&str>,
    ) -> Result<impl Reply, warp::Rejection> {

        let mut json_body = serde_json::to_string(body).map_err(|e| {
            warp::reject::custom(CustomException::new(
                &format!("Serde error: {}", e).to_string(),
            ))
        })?;

        ValidateIfAction::validate_class(path, &json_body)?;
    
        let is_token = path.contains(TOKEN_URL);
        let headers_config = match self.get_headers(is_token, custom_headers) {
            Ok(headers) => headers,
            Err(e) => {
                return Ok(reply::with_status(
                    reply::html(e.to_string()),
                    StatusCode::BAD_REQUEST,
                ));
            }
        };

        // Validación para encriptar
        if headers_config.contains_key("x-culqi-rsa-id") {
            if let Some(rsa_key) = &self.rsa_key {
                let encrypted_body = encrypt(&json_body, rsa_key, true).map_err(|e| {
                    warp::reject::custom(CustomException::new(
                        &format!("Serde error encrypt: {}", e).to_string(),
                    ));
                });
                json_body = serde_json::to_string(&encrypted_body).map_err(|e| {
                    warp::reject::custom(CustomException::new(
                        &format!("Serde error encrypt: {}", e).to_string(),
                    ))
                })?;

            } else {
                let error = CustomException::new("Se requiere una clave RSA");
                return Ok(reply::with_status(
                    reply::html(error.to_string()),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }

        let url = get_url(path, is_token);
        let response = match self
            .client
            .post(&url)
            .headers(headers_config)
            .body(json_body)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error en la solicitud POST: {}", e);
                return Ok(reply::with_status(
                    reply::html(e.to_string()),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };

        let status_code = response.status();
        let body = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error al leer el cuerpo de la respuesta: {}", e);
                return Ok(reply::with_status(
                    reply::html("Error al leer el cuerpo de la respuesta".to_string()),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };

        Ok(reply::with_status(reply::html(body), status_code))
    }

    pub async fn get(
        &self,
        path: &str,
        id: &str,
        custom_headers: Option<&str>,
    ) -> Result<(String, u16)> {
        let headers_config = match self.get_headers(false, custom_headers) {
            Ok(headers) => headers,
            Err(e) => {
                return Ok((e.to_string(), 400));
            }
        };

        let url = format!("{}/{}", get_url(path, false), id);
        let response = self.client.get(&url).headers(headers_config).send().await?;

        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok((body, status_code))
    }

    pub async fn all(
        &self,
        path: &str,
        params: &str,
        custom_headers: Option<&str>,
    ) -> Result<(String, u16)> {
        let headers_config = match self.get_headers(false, custom_headers) {
            Ok(headers) => headers,
            Err(e) => {
                return Ok((e.to_string(), 400));
            }
        };

        let json_value: Value = serde_json::from_str(params).unwrap();
        let query = json_value.as_object().map_or(String::new(), |obj| {
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

        let url = format!("{}?{}", get_url(path, false), query);
        print!("url: {}", url);
        let response = self.client.get(&url).headers(headers_config).send().await?;

        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok((body, status_code))
    }

    pub async fn delete(
        &self,
        path: &str,
        id: &str,
        custom_headers: Option<&str>,
    ) -> Result<(String, u16)> {
        let headers_config = match self.get_headers(false, custom_headers) {
            Ok(headers) => headers,
            Err(e) => {
                return Ok((e.to_string(), 400));
            }
        };

        let url = format!("{}/{}", get_url(path, false), id);
        let response = self
            .client
            .delete(&url)
            .headers(headers_config)
            .send()
            .await?;

        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok((body, status_code))
    }

    pub async fn patch<T: Serialize>(
        &self,
        path: &str,
        id: &str,
        body: &T,
        custom_headers: Option<&str>,
    ) -> Result<(String, u16)> {
        let json_body = serde_json::to_string(body)?;

        let headers_config = match self.get_headers(false, custom_headers) {
            Ok(headers) => headers,
            Err(e) => {
                return Ok((e.to_string(), 400));
            }
        };

        let url = format!("{}/{}", get_url(path, false), id);
        let response = self
            .client
            .patch(&url)
            .headers(headers_config)
            .body(json_body)
            .send()
            .await?;

        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok((body, status_code))
    }
}

fn get_url(path: &str, isTokenSecure: bool) -> String {
    let base_url = if isTokenSecure { SECURE_URL } else { BASE_URL };
    format!("{}{}", base_url, path)
}
