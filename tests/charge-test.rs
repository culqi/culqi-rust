use tokio;
use LibCulqi::*;
mod config; // Esto importa el archivo config.rs en la carpeta tests
use config::credentials::{PUBLIC_KEY, SECRET_KEY,RSA_KEY, RSA_ID};
mod request { 
    pub mod charge;
}

use request::charge::{REQUEST_CHARGUE_BODY,CUSTOM_HEADERS};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[tokio::test]
    async fn test_cargo_create() {
        match create(REQUEST_CHARGUE_BODY, "charges", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_cargo_create_with_custom_headers() {
        match create_with_custom_headers(
            REQUEST_CHARGUE_BODY,
            "charges",
            PUBLIC_KEY,
            SECRET_KEY,
            CUSTOM_HEADERS,
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
    async fn test_cargo_encrypt_create_with_custom_headers() {
        match create_encrypt_with_custom_headers(
            REQUEST_CHARGUE_BODY,
            "charges",
            PUBLIC_KEY,
            SECRET_KEY,
            RSA_KEY,
            RSA_ID,
            CUSTOM_HEADERS,
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
  
}
