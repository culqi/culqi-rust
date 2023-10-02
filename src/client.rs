/*
use std::error::Error;
//use reqwest::{Client, RequestBuilder};
mod encrypt;
use encrypt::encrypt;
use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
//use serde_json::json;

const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
const BASE_URL: &str = "https://api.culqi.com/v2/";

const PUBLIC_KEY: &str = "INSERTE_SU_LLAVE_PUBLICA";
const SECRET_KEY: &str = "INSERTE_SU_LLAVE_PRIVADA";

const RSA_ID: &str = "INSERTE_SU_RSA_ID";
const RSA_KEY: &str = "INSERTE_SU_LLAVE_RSA";
*/
/*
pub struct Client {

}
*/
//extern crate encrypt;
pub mod Client {
    use std::error::Error;
    
    mod encrypt;
    use encrypt::encrypt;
    use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};

    const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
const BASE_URL: &str = "https://api.culqi.com/v2/";

const PUBLIC_KEY: &str = "INSERTE_SU_LLAVE_PUBLICA";
const SECRET_KEY: &str = "INSERTE_SU_LLAVE_PRIVADA";

const RSA_ID: &str = "INSERTE_SU_RSA_ID";
const RSA_KEY: &str = "INSERTE_SU_LLAVE_RSA";

pub async fn create(
    body: &str,
    action: &str,
    pkey: &str,
    skey: &str,
    rsa_pid: &str,
    rsa_pkey: &str,
) -> Result<(String, u16), Box<dyn Error>> {

    let key: &str;
    let url: String;
    let request_body: String;

    if rsa_pid.len() != 0 && rsa_pkey.len() != 0  {
        let encrypt = encrypt(body, rsa_pkey, true)?;
        request_body = serde_json::to_string(&encrypt).unwrap()
    }else{
        request_body = body.to_owned();
    }

    println!("request_body: {:?}", request_body);
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
        .body(request_body)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

    Ok((response_text, status_code))
}

pub async fn patch(
    body: &str,
    action: &str,
    pkey: &str,
    pid: &str,
    skey: &str,
    rsa_pid: &str,
    rsa_pkey: &str,
) -> Result<(String, u16), Box<dyn Error>> {

    let key: &str;
    let url: String;
    let request_body: String;

    if rsa_pid.len() != 0 && rsa_pkey.len() != 0  {
        let encrypt = encrypt(body, rsa_pkey, true)?;
        request_body = serde_json::to_string(&encrypt).unwrap()
    }else{
        request_body = body.to_owned();
    }

    println!("request_body: {:?}", request_body);
    println!("action: {:?}", action);

    url = BASE_URL.to_owned() + action + "/" + pid;
    key = SECRET_KEY;
    println!("key: {:?}", key);

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_pid).parse().unwrap());

    // Añade aquí cualquier otro encabezado que necesites

    let response = client.post(&url)
        .headers(headers)
        .body(request_body)
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
}
//--------------fin funciones- genericas---------------

//--------------test---------------
/*
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
/*
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
*/
    #[tokio::test]
    async fn test_token_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_TOKEN_BODY, "tokens", PUBLIC_KEY, SECRET_KEY,"","").await {
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

        match create(REQUEST_TOKEN_BODY, "tokens", PUBLIC_KEY, SECRET_KEY, RSA_ID, RSA_KEY).await  {
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

        match create(REQUEST_CHARGUE_BODY, "charges", PUBLIC_KEY, SECRET_KEY,"","").await {
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
}*/