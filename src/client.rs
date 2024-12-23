use anyhow::Result;
use hyper::header::HeaderName;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;

use crate::{
    utils::{
        encrypt::encrypt,
        urls::{get_url, TOKEN_URL},
        CustomException::CustomException,
    },
    validation::validate_if_action::ValidateIfAction,
};
pub struct Client {
    secret_key: String,
    public_key: String,
    rsa_key:    Option<String,>,
    client:     reqwest::Client,
}

impl Client {
    pub fn config(secret_key: &str, public_key: &str, rsa_key: Option<&str,>,) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            rsa_key:    rsa_key.map(|key| key.to_string(),),
            client:     reqwest::Client::new(),
        }
    }

    fn get_headers(
        &self,
        isToken: bool,
        custom_headers: Option<Value,>,
    ) -> Result<HeaderMap, CustomException,> {
        const X_CULQI_ENV_TEST: &str = "test";
        const X_CULQI_ENV_LIVE: &str = "live";
        const X_CULQI_CLIENT: &'static str = "culqi-rust";
        const X_CULQI_CLIENT_VERSION: &'static str = "1.0.1";
        const X_API_VERSION: &'static str = "2";

        let mut headers = HeaderMap::new();

        let token = if isToken {
            &self.public_key
        } else {
            &self.secret_key
        };

        let env = match token.contains("test",) {
            true => X_CULQI_ENV_TEST,
            false => X_CULQI_ENV_LIVE,
        };

        headers.insert(
            "x-culqi-env",
            HeaderValue::from_str(env,).expect("Failed to parse x-culqi-env",),
        );

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token),).unwrap(),
        );

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8",),
        );

        headers.insert(
            "x-api-version",
            X_API_VERSION.parse().expect("Failed to parse x-api-version",),
        );
        headers.insert(
            "x-culqi-client",
            X_CULQI_CLIENT.parse().expect("Failed to parse x-culqi-client",),
        );
        headers.insert(
            "x-culqi-client-version",
            X_CULQI_CLIENT_VERSION.parse().expect("Failed to parse x-culqi-client-version",),
        );

        if let Some(custom_headers_value,) = custom_headers {
            if let Some(custom_headers_map,) = custom_headers_value.as_object() {
                for (header_key, header_value,) in custom_headers_map {
                    let header_value_str = match header_value {
                        Value::String(s,) => s.clone(),
                        Value::Number(n,) => n.to_string(),
                        Value::Bool(b,) => b.to_string(),
                        _ => String::new(), // Caso por defecto si el tipo es desconocido
                    };

                    if !header_value_str.is_empty() {
                        if let Ok(header_name,) = HeaderName::from_bytes(header_key.as_bytes(),) {
                            if let Ok(header_value,) = HeaderValue::from_str(&header_value_str,) {
                                headers.insert(header_name, header_value,);
                            }
                        }
                    }
                }
            }
        }

        Ok(headers,)
    }

    pub async fn post<T: Serialize,>(
        &self,
        path: &str,
        body: &T,
        custom_headers: Option<Value,>,
    ) -> Result<(String, u16,), (String, u16,),> {
        let mut json_body = serde_json::to_string(body,).map_err(|_| {
            let error = CustomException::new(&"Error al convertir el cuerpo a JSON",);
            (error.to_string(), 400,)
        },)?;

        if let Err(validation_error,) = ValidateIfAction::validate_class(path, &json_body,) {
            return Err((validation_error.to_string(), 400,),);
        }

        let isToken = path.contains(TOKEN_URL,);
        let headers_config = match self.get_headers(isToken, custom_headers,) {
            Ok(headers,) => headers,
            Err(e,) => {
                return Err((e.to_string(), 400,),);
            }
        };

        // Validacion para encriptar
        if headers_config.contains_key("x-culqi-rsa-id",) {
            if let Some(rsa_key,) = &self.rsa_key {
                let encrypted_body = encrypt(&json_body, rsa_key, true,).map_err(|_| {
                    let error = CustomException::new(&"Error al encriptar:",);
                    (error.to_string(), 400,)
                },)?;

                json_body = serde_json::to_string(&encrypted_body,).map_err(|_| {
                    let error = CustomException::new(&"Error al convertir el cuerpo a JSON",);
                    (error.to_string(), 400,)
                },)?;
            } else {
                let error = CustomException::new("Se requiere una clave RSA",);
                return Err((error.to_string(), 400,),);
            }
        }

        let url = get_url(path, isToken,);
        let response =
            self.client.post(&url,).headers(headers_config,).body(json_body,).send().await;

        let response = match response {
            Ok(resp,) => resp,
            Err(_,) => {
                print!("Erro Client");
                let error = CustomException::new("Error en la solicitud",);
                return Err((error.to_string(), 400,),);
            }
        };

        let status_code = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "".to_string(),);

        match status_code {
            200..=299 => Ok((body, status_code,),),
            _ => Err((body, status_code,),),
        }
    }

    pub async fn get(
        &self,
        path: &str,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<(String, u16,), (String, u16,),> {
        if let Err(validation_error,) = ValidateIfAction::validate_id_class(path, &id,) {
            return Err((validation_error.to_string(), 400,),);
        }

        let headers_config = match self.get_headers(false, custom_headers,) {
            Ok(headers,) => headers,
            Err(e,) => {
                return Err((e.to_string(), 400,),);
            }
        };

        let url = format!("{}/{}", get_url(path, false), id);
        let response = self.client.get(&url,).headers(headers_config,).send().await;
        let response = match response {
            Ok(resp,) => resp,
            Err(_,) => {
                print!("Erro Client");
                let error = CustomException::new("Error en la solicitud",);
                return Err((error.to_string(), 400,),);
            }
        };

        let status_code = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "".to_string(),);

        match status_code {
            200..=299 => Ok((body, status_code,),),
            _ => Err((body, status_code,),),
        }
    }

    pub async fn all<T: Serialize,>(
        &self,
        path: &str,
        params: &T,
        custom_headers: Option<Value,>,
    ) -> Result<(String, u16,), (String, u16,),> {
        let json_body = serde_json::to_string(params,).map_err(|_| {
            let error = CustomException::new(&"Error al convertir el cuerpo a JSON",);
            (error.to_string(), 400,)
        },)?;
        if let Err(validation_error,) = ValidateIfAction::validate_all_class(path, &json_body,) {
            return Err((validation_error.to_string(), 400,),);
        }

        let headers_config = match self.get_headers(false, custom_headers,) {
            Ok(headers,) => headers,
            Err(e,) => {
                return Err((e.to_string(), 400,),);
            }
        };

        let json_value: Value = match serde_json::to_value(params,) {
            Ok(value,) => value,
            Err(e,) => return Err((e.to_string(), 400,),),
        };

        let query = json_value.as_object().map_or(String::new(), |obj| {
            obj.iter()
                .map(|(key, value,)| {
                    if let Some(string_value,) = value.as_str() {
                        format!("{}={}", key, string_value)
                    } else {
                        format!("{}={}", key, value.to_string().trim_matches('"'))
                    }
                },)
                .collect::<Vec<String,>>()
                .join("&",)
        },);

        let url = format!("{}?{}", get_url(path, false), query);
        let response = self.client.get(&url,).headers(headers_config,).send().await;
        let response = match response {
            Ok(resp,) => resp,
            Err(_,) => {
                print!("Erro Client");
                let error = CustomException::new("Error en la solicitud",);
                return Err((error.to_string(), 400,),);
            }
        };

        let status_code = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "".to_string(),);
        match status_code {
            200..=299 => Ok((body, status_code,),),
            _ => Err((body, status_code,),),
        }
    }

    pub async fn delete(
        &self,
        path: &str,
        id: &str,
        custom_headers: Option<Value,>,
    ) -> Result<(String, u16,), (String, u16,),> {
        if let Err(validation_error,) = ValidateIfAction::validate_id_class(path, &id,) {
            return Err((validation_error.to_string(), 400,),);
        }

        let headers_config = match self.get_headers(false, custom_headers,) {
            Ok(headers,) => headers,
            Err(e,) => {
                return Err((e.to_string(), 400,),);
            }
        };

        let url = format!("{}/{}", get_url(path, false), id);
        let response = self.client.delete(&url,).headers(headers_config,).send().await;
        let response = match response {
            Ok(resp,) => resp,
            Err(_,) => {
                print!("Erro Client");
                let error = CustomException::new("Error en la solicitud",);
                return Err((error.to_string(), 400,),);
            }
        };

        let status_code = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "".to_string(),);
        match status_code {
            200..=299 => Ok((body, status_code,),),
            _ => Err((body, status_code,),),
        }
    }

    pub async fn patch<T: Serialize,>(
        &self,
        path: &str,
        id: &str,
        body: &T,
        custom_headers: Option<Value,>,
    ) -> Result<(String, u16,), (String, u16,),> {
        let json_body = serde_json::to_string(body,).map_err(|_| {
            let error = CustomException::new(&"Error al convertir el cuerpo a JSON",);
            (error.to_string(), 400,)
        },)?;

        if let Err(validation_error,) =
            ValidateIfAction::validate_update_class(path, &id, &json_body,)
        {
            return Err((validation_error.to_string(), 400,),);
        }

        let headers_config = match self.get_headers(false, custom_headers,) {
            Ok(headers,) => headers,
            Err(e,) => {
                return Ok((e.to_string(), 400,),);
            }
        };

        let url = format!("{}/{}", get_url(path, false), id);
        let response =
            self.client.patch(&url,).headers(headers_config,).body(json_body,).send().await;

        let response = match response {
            Ok(resp,) => resp,
            Err(_,) => {
                print!("Erro Client");
                let error = CustomException::new("Error en la solicitud",);
                return Err((error.to_string(), 400,),);
            }
        };

        let status_code = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "".to_string(),);
        match status_code {
            200..=299 => Ok((body, status_code,),),
            _ => Err((body, status_code,),),
        }
    }
}
