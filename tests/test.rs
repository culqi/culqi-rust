use LibCulqi28::{create,get,delete};
//mod token;
//use LibCulqi28::token;
use crate::src::token;
//use std::error::Error;
//use reqwest::{Client, RequestBuilder};
//mod src;
//mod encrypt;
//use crate::src::encrypt;

const PUBLIC_KEY: &str = "pk_test_e94078b9b248675d";
const SECRET_KEY: &str = "sk_test_c2267b5b262745f0";

const RSA_ID: &str = "de35e120-e297-4b96-97ef-10a43423ddec";
const RSA_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDswQycch0x/7GZ0oFojkWCYv+g
r5CyfBKXc3Izq+btIEMCrkDrIsz4Lnl5E3FSD7/htFn1oE84SaDKl5DgbNoev3pM
C7MDDgdCFrHODOp7aXwjG8NaiCbiymyBglXyEN28hLvgHpvZmAn6KFo0lMGuKnz8
HiuTfpBl6HpD6+02SQIDAQAB
-----END PUBLIC KEY-----";




//--------------fin funciones- genericas---------------

//--------------test---------------

const REQUEST_TOKEN_BODY : &str = r#"{
    "card_number": "4111111111111111",
    "cvv": "123",
    "expiration_month": "09",
    "expiration_year": "2025",
    "email": "alexis.pumayalla@culqi.com",
    "metadata": {
        "coment": "Tarjeta de prueba alexis"
    }
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_TOKEN_BODY, "tokens", PUBLIC_KEY, SECRET_KEY, "", "").await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
/*
    #[tokio::test]
    async fn test_token_encrypt() {
        // Ejemplo de cómo usar la función

        match create_encrypt(REQUEST_TOKEN_BODY, "tokens", PUBLIC_KEY, SECRET_KEY, RSA_KEY, RSA_ID).await  {
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
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    const REQUEST_CHARGUE_BODY : &str = r#"{
        "amount": 600,
        "currency_code": "PEN",
        "email": "review@culqi.com",
        "source_id": "tkn_test_IctezQFcWKhvOHyQ",
        "antifraud_details": {
            "first_name": "Fernando",
            "last_name": "Chullo",
            "email": "review134@culqi.com",
            "phone_number": "945737476",
            "device_finger_print_id": "8b17f1dc-e616-46cf-b416-ec7ef63730e9"
        }
    }"#;

    #[tokio::test]
    async fn test_cargo_create() {
        // Ejemplo de cómo usar la función

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
    async fn test_order_delete() {

        match delete("tokens", "ord_test_20HjpSkdDlSdoHEC", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }*/
}