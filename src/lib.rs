use std::error::Error;
use reqwest::{Client, RequestBuilder};
mod encrypt;
use encrypt::encrypt;

const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
const BASE_URL: &str = "https://api.culqi.com/v2/";

const PUBLIC_KEY: &str = "pk_test_e94078b9b248675d";
const SECRET_KEY: &str = "sk_test_c2267b5b262745f0";

const RSA_ID: &str = "de35e120-e297-4b96-97ef-10a43423ddec";
const RSA_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDswQycch0x/7GZ0oFojkWCYv+g
r5CyfBKXc3Izq+btIEMCrkDrIsz4Lnl5E3FSD7/htFn1oE84SaDKl5DgbNoev3pM
C7MDDgdCFrHODOp7aXwjG8NaiCbiymyBglXyEN28hLvgHpvZmAn6KFo0lMGuKnz8
HiuTfpBl6HpD6+02SQIDAQAB
-----END PUBLIC KEY-----";


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct ResponseData {
    pub text: String,
    pub status_code: u16,
}


//--------------funciones- genericas---------------
pub async fn create(
    body: &str,
    action: &str,
    pk: &str,
    sk: &str,
) -> Result<(String, u16), reqwest::Error> {
    let skey: &str = sk;
    let pkey: &str = pk;

    let key: &str;
    let url: String;

    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        url = BASE_URL.to_owned() + action;
    }

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", key))
        .body(body.to_owned()) // Clonar el contenido del body a un String propio
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;


    Ok((response_text, status_code))
}

use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;

pub async fn create_encrypt(
    body: &str,
    action: &str,
    pkey: &str,
    skey: &str,
    rsa_pkey: &str,
    rsa_pid: &str,
) -> Result<(String, u16), Box<dyn Error>> {

    let body_encrypt = encrypt(body, rsa_pkey, true)?;

    println!("body_encrypt: {:?}", body_encrypt);

    let key: &str;
    let url: String;

    println!("action: {:?}", action);

    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        url = BASE_URL.to_owned() + action;
    }

    println!("key: {:?}", key);

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_pid).parse().unwrap());

    // Añade aquí cualquier otro encabezado que necesites

    let response = client.post(&url)
        .headers(headers)
        .body(serde_json::to_string(&body_encrypt).unwrap())
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn get(
    action: &str,
    query: &str,
    skey: &str,
) -> Result<String, reqwest::Error> {
    let key: &str;
    let url: String;

    key = skey;
    url = BASE_URL.to_owned() + action + "/" + query;

    let client = reqwest::Client::new();

    let response = client.get(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .send()
        .await?;

    let response_text = response.text().await?;

    Ok(response_text)
}

pub async fn delete(
    action: &str,
    query: &str,
    skey: &str,
) -> Result<String, reqwest::Error> {
    let key: &str;
    let url: String;

    key = skey; // Asegúrate de definir skey
    url = BASE_URL.to_owned() + action + "/" + query; // Asegúrate de definir BASEURL

    let client = reqwest::Client::new();

    let response = client.delete(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .send()
        .await?;

    let response_text = response.text().await?;

    Ok(response_text)
}

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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_token_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_TOKEN_BODY, "tokens", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

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
    }
}