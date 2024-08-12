use core::fmt;
use std::{error::Error, fmt::Debug};
use reqwest::{Client, RequestBuilder};
mod encrypt;
use encrypt::encrypt;
mod validation;
mod headers;
use headers::CustomHeaders;
use validation::helpers::Helpers;
use validation::validate_if_action::ValidateIfAction;

use anyhow::Result;

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
        if(action == "plans" || action == "subscriptions"){
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        }else {
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
    custom_headers: &str
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
        if(action == "plans" || action == "subscriptions"){
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        }else {
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
        if(action == "plans" || action == "subscriptions"){
            url = BASE_URL.to_owned() + "recurrent/" + action + "/" + query;
        }else {
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

use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use serde_json::{json, Value};

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

    println!("body_encrypt: {:?}", body_encrypt);

    let key: &str;
    let url: String;

    println!("action: {:?}", action);
    
    if action == "tokens" {
        key = pkey;
        url = SECURE_URL.to_string();
    } else {
        key = skey;
        if(action == "plans" || action == "subscriptions"){
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        }else {
            url = BASE_URL.to_owned() + action;
        }
    }

    println!("key: {:?}", key);

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_pid).parse().unwrap());

    let additional_headers = CustomHeaders::get_headers(key);

    // Añade aquí cualquier otro encabezado que necesites

    let response = client.post(&url)
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
    custom_headers: &str
) -> Result<(String, u16), Box<dyn Error>> {
    ValidateIfAction::validate_class(action, body)?;

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
        if(action == "plans" || action == "subscriptions"){
            url = BASE_URL.to_owned() + "recurrent/" + action + "/create";
        }else {
            url = BASE_URL.to_owned() + action;
        }
    }

    println!("key: {:?}", key);

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_pid).parse().unwrap());

    let additional_headers = CustomHeaders::get_custom_headers(key, custom_headers);

    // Añade aquí cualquier otro encabezado que necesites

    let response = client.post(&url)
        .headers(headers)
        .headers(additional_headers)
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
) -> Result<(String, u16)> {
    let key: &str;
    let url: String;
    ValidateIfAction::validate_id_class(action, query)?;
    key = skey;
    if(action == "plans" || action == "subscriptions"){
        url = BASE_URL.to_owned() + "recurrent/" + action + "/" + query;
    }else {
        url = BASE_URL.to_owned() + action + "/" + query;
    }

    let client = reqwest::Client::new();

    let additional_headers = CustomHeaders::get_headers(key);

    let response = client.get(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .headers(additional_headers)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;


    Ok((response_text, status_code))
}

pub async fn all(
    action: &str,
    query: &str,
    skey: &str,
) -> Result<(String, u16)> {
    let key: &str;
    let url: String;
    ValidateIfAction::validate_all_class(action, query)?;

    let json_value: Value = serde_json::from_str(query).unwrap();
    let query_string = json_value
    .as_object()
    .map_or(String::new(), |obj| {
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
    if(action == "plans" || action == "subscriptions"){
        url = BASE_URL.to_owned() + "recurrent/" + action + "?"  + &query_string;
        print!("{}", url)
    }else {
        url = BASE_URL.to_owned() + action + "?" + &query_string;
    }

    let client = reqwest::Client::new();

    let additional_headers = CustomHeaders::get_headers(key);

    let response = client.get(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .headers(additional_headers)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let response_text = response.text().await?;

   Ok((response_text, status_code))
}

pub async fn delete(
    action: &str,
    query: &str,
    skey: &str,
) ->  Result<(String, u16)>  {
    let key: &str;
    let url: String;
    ValidateIfAction::validate_id_class(action, query)?;
    key = skey; // Asegúrate de definir skey
    if(action == "plans" || action == "subscriptions"){
        url = BASE_URL.to_owned() + "recurrent/" + action + "/" + query;
    }else {
        url = BASE_URL.to_owned() + action + "/" + query;
    }

    let client = reqwest::Client::new();
    let additional_headers = CustomHeaders::get_headers(key);

    let response = client.delete(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .headers(additional_headers)
        .send()
        .await?;

    
    let status_code = response.status().as_u16();
    let response_text = response.text().await?;
    
    
    Ok((response_text, status_code))
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
            Err(err) => println!("{:?}", err),
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
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_PLAN_CREATE, "plans", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_plan_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_PLAN_UPDATE, "plans", "pln_live_oUr3os1vYacQ4wI8", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 200, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_plan_get() {

        match get("plans", "pln_live_oUr3os1vYacQ4wI8", SECRET_KEY).await {
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_all() {

        match all("plans", REQUEST_PLAN_ALL, SECRET_KEY).await {
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_delete() {

        match delete("plans", "pln_live_oUr3os1vYacQ8wI9", SECRET_KEY).await {
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    

    #[tokio::test]
    async fn test_subscription_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_SUBSCRIṔTTION_CREATE, "subscriptions", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_subscription_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_SUBSCRIṔTTION_UPDATE, "subscriptions", "sxn_live_neFrhLr8QvozBdWn", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 200, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_subscription_all() {

        match all("subscriptions", REQUEST_SUBSCRIṔTTION_ALL, SECRET_KEY).await {
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_subscription_get() {

        match get("subscriptions", "sxn_live_neFrhLrX8vozBdWn", SECRET_KEY).await {
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_subscription_delete() {

        match delete("subscriptions", "sxn_live_neFrh8rXQvozBdWn", SECRET_KEY).await {
            Ok((response_text, status_code))=> println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    const REQUEST_PLAN_CREATE : &str = r#"{
        "short_name": "cp-prueb2442",
        "description": "Cypress PCI | ERRROR NO USAR",
        "amount": 300,
        "currency": "PEN",
        "interval_unit_time": 1,
        "interval_count": 1,
        "initial_cycles": {
          "count": 1,
          "has_initial_charge": true,
          "amount": 400,
          "interval_unit_time": 1
        },
        "name": "CY PCI - ERROR 100018",
        "image": "https://recurrencia-suscripciones-qa.s3.amazonaws.com/f097e1d5-e365-42f3-bc40-a27beab80f54",
	    "metadata":{
	    	"key": "value"
	    }
    }"#;

    const REQUEST_PLAN_UPDATE : &str = r#"{
        "short_name": "cp-prueb2442",
        "description": "Cypress PCI | ERRROR NO USAR",
        "name": "CY PCI - ERROR 100018",
        "image": "https://recurrencia-suscripciones-qa.s3.amazonaws.com/f097e1d5-e365-42f3-bc40-a27beab80f54"
    }"#;

    const REQUEST_PLAN_ALL: &str = r#"{
        "limit": 100,
        "status": 1,
        "before": "pln_live_oUr88s1vYacQ4wI9"
    }"#;

    const REQUEST_SUBSCRIṔTTION_CREATE : &str = r#"{
        "plan_id": "pln_live_oUr3os18YacQ4wI9",
        "card_id": "crd_live_oUr3os1vYacQ4wI9",
        "tyc": true,
        "metdata": {
            "key": "value"
        }
    }"#;

    const REQUEST_SUBSCRIṔTTION_UPDATE : &str = r#"{
        "card_id": "crd_live_oUr88s1vYacQ4wI9",
        "metdata": {
            "key": "value"
        }
    }"#;

    const REQUEST_SUBSCRIṔTTION_ALL: &str = r#"{
        "limit": 100,
        "status": 1,
        "plan_id": "pln_live_oUr3os18YacQ4wI9"
    }"#;


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

    const CUSTOM_HEADERS : &str = r#"{
        "X-Charge-Channel": null,
        "X-Plan-Type": 1,
        "X-Header-Config": true
    }"#;

    #[tokio::test]
    async fn test_cargo_create_with_custom_headers() {
        match create_with_custom_headers(REQUEST_CHARGUE_BODY, "charges", PUBLIC_KEY, SECRET_KEY, CUSTOM_HEADERS).await {
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
        match create_encrypt_with_custom_headers(REQUEST_CHARGUE_BODY, "charges", PUBLIC_KEY, SECRET_KEY,RSA_KEY, RSA_ID, CUSTOM_HEADERS).await {
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
            Ok((response_text, status_code)) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}