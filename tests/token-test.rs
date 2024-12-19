use tokio;
use BrandoCulqi::*;
mod config;
use config::credentials::{PUBLIC_KEY, RSA_ID, RSA_KEY, SECRET_KEY};
use BrandoCulqi::client::Client;
use BrandoCulqi::culqi::token::Token;

mod request;
use request::token::{
    create_token_request, create_token_yape_request, update_token_request, LIST_TOKEN_REQUEST,
};

use lazy_static::lazy_static;
lazy_static! {
    static ref client: Client = Client::config(SECRET_KEY, PUBLIC_KEY, Some(RSA_KEY));
}
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_token_create() {
        match Token::create(&client, &create_token_request(), None).await {
            Ok((response_text, status_code)) => {
                println!("Token creado exitosamente con status code: {}", status_code);
                println!("Respuesta del token: {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(e) => {
                println!("Error al crear el token: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_token_create_encrypt() {
        const CUSTOM_HEADERS: &str = r#"{
            "x-culqi-rsa-id": "{RSA_ID}"
        }"#;
        let custom_headers = CUSTOM_HEADERS.replace("{RSA_ID}", RSA_ID);
        match Token::create(&client, &create_token_request(), Some(&custom_headers)).await {
            Ok((response_text, status_code)) => {
                println!(
                    "Token Encrypt creado exitosamente con status code: {}",
                    status_code
                );
                println!("Respuesta del token: {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(e) => {
                println!("Error al crear el token encriptado: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_token_get() {
        let token = Token::create(&client, &create_token_request(), None).await;
        match token {
            Ok((response_text, _status_code)) => {
                println!("Respuesta crear Token: {}", response_text);
                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");
                if let Some(token_id) = response_json["id"].as_str() {
                    println!("ID del token: {}", token_id);
                    match Token::get(&client, token_id, None).await {
                        Ok((response_text, _status_code)) => {
                            println!("Respuesta del GET token: {}", response_text);
                            assert_eq!(response_json["object"], "token");
                            assert!(
                                response_json["id"].is_string(),
                                "El campo 'id' no es una cadena"
                            );
                        }
                        Err(e) => eprintln!("Error al obtener el token: {}", e),
                    }
                } else {
                    eprintln!("No se encontró el 'id' en la respuesta del servidor");
                }
            }
            Err(e) => eprintln!("Error al crear el token: {}", e),
        }
    }

    #[tokio::test]
    async fn test_token_list() {
        match Token::all(&client, LIST_TOKEN_REQUEST, None).await {
            Ok((response_text, status_code)) => {
                println!(
                    "Token Encrypt creado exitosamente con status code: {}",
                    status_code
                );
                println!("Respuesta del token: {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser 'object'"
                );
            }
            Err(e) => {
                println!("Error al crear el token encriptado: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_token_update() {
        let token = Token::create(&client, &create_token_request(), None).await;
        match token {
            Ok((response_text, _status_code)) => {
                println!("Respuesta crear Token: {}", response_text);
                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");
                if let Some(token_id) = response_json["id"].as_str() {
                    println!("ID del token: {}", token_id);
                    match Token::patch(&client, token_id, &update_token_request(), None).await {
                        Ok((response_text, _status_code)) => {
                            println!("Respuesta del UPDATE token: {}", response_text);
                            assert_eq!(response_json["object"], "token");
                            assert!(
                                response_json["id"].is_string(),
                                "El campo 'id' no es una cadena"
                            );
                        }
                        Err(e) => eprintln!("Error al obtener el token: {}", e),
                    }
                } else {
                    eprintln!("No se encontró el 'id' en la respuesta del servidor");
                }
            }
            Err(e) => eprintln!("Error al crear el token: {}", e),
        }
    }

    #[tokio::test]
    async fn test_token_yape_create() {
        match Token::yape(&client, &create_token_yape_request(), None).await {
            Ok((response_text, status_code)) => {
                println!("Token creado exitosamente con status code: {}", status_code);
                println!("Respuesta del token: {}", response_text);

                let response_json: Value = serde_json::from_str(&response_text)
                    .expect("Error al parsear la respuesta JSON");

                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(e) => {
                println!("Error al crear el token: {}", e);
            }
        }
    }

}
