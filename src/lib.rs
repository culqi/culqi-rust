mod Encrypt;
mod Culqi;
use Culqi::create;
use Culqi::add;
use Culqi::createEncrypt;
use Culqi::delete;
use Culqi::get;
use Encrypt::encrypt;

use isahc::prelude::*;
use std::convert::TryInto;
use std::error::Error;
use std::io::Read;
use rsa::pkcs8::der::Decodable;
use rsa::pkcs1::FromRsaPublicKey;
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use rsa::{PublicKey};



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_create() {
        // Ejemplo de cómo usar la función
        let body = r#"{
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

        match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }



    #[test]
    fn test_token() {
        // Ejemplo de cómo usar la función
        let body = r#"{
            "card_number": "4111111111111111",
            "cvv": "123",
            "expiration_month": "09",
            "expiration_year": "2025",
            "email": "alexis.pumayalla@culqi.com",
            "metadata": {
                "coment": "Tarjeta de prueba alexis"
            }
        }"#;

        match create(body, "tokens") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_tokenEncrypt() {
        // Ejemplo de cómo usar la función
        let body = "{\"card_number\":\"4111111111111111\",\"cvv\":\"123\",\"expiration_month\":\"09\",\"expiration_year\":\"2025\",\"email\":\"alexis.pumayalla@culqi.com\",\"metadata\":{\"coment\":\"Tarjeta de prueba alexis\"}}";


        match createEncrypt(body, "tokens") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_token_get() {

        match get("tokens", "tkn_test_20HjpSkdDlSdoHEC") {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[test]
    fn test_order_delete() {

        match delete("tokens", "ord_test_20HjpSkdDlSdoHEC") {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[test]
    fn test_update_token() {

            let body = r#"{
                "metadata": {
                   "dni": "5831543",
                   "cliente_id": 259
                   }
            }"#;

            match update("tokens", "tkn_test_20HjpSkdDlSdoHEC" ,body) {
                Ok((response_text, status_code)) => {
                    println!("Status Code: {}", status_code);
                    println!("Response Text: {}", response_text);
                }
                Err(err) => println!("Error: {:?}", err),
            }
    }
}
