use tokio;
use LibCulqi::*;
mod config; // Esto importa el archivo config.rs en la carpeta tests
use config::credentials::{PUBLIC_KEY, SECRET_KEY,RSA_KEY, RSA_ID};
mod request { 
    pub mod token;
}

use request::token::REQUEST_TOKEN_BODY;

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
        match create(REQUEST_TOKEN_BODY, "tokens", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_token_encrypt() {
        // Ejemplo de cómo usar la función

        match create_encrypt(
            REQUEST_TOKEN_BODY,
            "tokens",
            PUBLIC_KEY,
            SECRET_KEY,
            RSA_KEY,
            RSA_ID,
        )
        .await
        {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_token_get() {
        match get("tokens", "tkn_test_20HjpSkdDlSdoHEC", SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
