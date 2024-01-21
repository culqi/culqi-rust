use std::{error::Error, fmt::Debug};
use reqwest::{Client, RequestBuilder};
mod encrypt;
use encrypt::encrypt;
mod validation;
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

pub async fn update(
    body: &str,
    action: &str,
    pk: &str,
    sk: &str,
) -> Result<(String, u16)> {
    let skey: &str = sk;
    let pkey: &str = pk;

    let key: &str;
    let url: String;
    key = skey;
    url = BASE_URL.to_owned() + action;

    let client = Client::new();
    let response = client
        .patch(url)  // Cambiado de .post() a .patch()
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", key))
        .body(body.to_owned()) // Clonar el contenido del body a un String propio
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


const REQUEST_YAPE_BODY : &str = r#"{
    "otp": "946627",
    "number_phone": "951123456",
    "amount": "500",
    "metadata": {
    "dni": "5831543"
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
    async fn test_token_create_yape() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_YAPE_BODY, "tokens/yape", PUBLIC_KEY, SECRET_KEY).await {
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
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_token_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_TOKEN_UPDATE_BODY, "tokens/tkn_test_20HjpSkdDlSdoHEC", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
    const REQUEST_REFUND_BODY : &str = r#"{
        "amount": 600,
        "charge_id": "chr_test_3xWxRF1Zswgp6C7N",
        "reason": "fraudulento"
    }"#;

    const REQUEST_CLIENT_BODY : &str = r#"{
        "first_name": "Richard",
        "last_name": "Hendricks",
        "email": "richard@piedpiper.com",
        "address": "San Francisco Bay Area",
        "address_city": "Palo Alto",
        "country_code": "US",
        "phone_number": "6505434800"
    }"#;

    const REQUEST_PLAN_BODY : &str = r#"{
       "name": "Plan de Prueba.",
        "short_name": "plan-de-prueba-001",
        "description": "Descripción Plan de Prueba",
        "amount": 5,
        "currency": "PEN",
        "interval_unit_time": 1,
        "interval_count": 1,
        "initial_cycles": {
        "count": 0,
        "has_initial_charge": false,
        "amount": 0,
        "interval_unit_time": 1
        },
        "metadata": {
        "DNI": 123456782
        }
    }"#;

    const REQUEST_SUSCRIPCION_BODY : &str = r#"{
        "card_id": "string",
        "plan_id": "string",
        "tyc": true,
        "metadata": { }
    }"#;
    const REQUEST_CARD_BODY : &str = r#"{
        "customer_id": "cus_test_Lz6Yfsm7QqCPIECW",
        "token_id": "tkn_test_vEcZSCOVz5PGDPdQ",
        "validate": true,
        "metadata": {
        "marca_tarjeta": "VISA"
        }
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

    const REQUEST_ORDER_BODY: &str = r#"{
        "amount": 60000,
        "currency_code": "PEN",
        "description": "Venta de polo",
        "order_number": "id-9999",
        "expiration_date": "1476132639",
        "client_details": {
            "first_name": "Richard",
            "last_name": "Hendricks",
            "email": "richard@piedpiper.com",
            "phone_number": "999999987"
        },
        "confirm": true,
        "metadata": {
            "dni": "71702999"
        }
    }"#;


    const REQUEST_REFUND_UPDATE_BODY : &str = r#"{
        "metadata": {
            "dni": "71701978"
        }
    }"#;

    const REQUEST_ORDER_UPDATE_BODY : &str = r#"{
        "expiration_date": 1661117022,
        "metadata": {
        "dni": "71701978"
        }
    }"#;

    const REQUEST_CHARGE_UPDATE_BODY : &str = r#"{
        "metadata": {
        "documentType": "1",
        "documentNumber": "99999999"
        }
    }"#;

    const REQUEST_TOKEN_UPDATE_BODY : &str = r#"{
        "metadata": {
        "dni": "5831543",
        "cliente_id": 259
        }
    }"#;

    const REQUEST_PLAN_UPDATE_BODY : &str = r#"{
        "name": "Nuevo Plan de Prueba",
        "short_name": "plan-nombre-del-plan",
        "description": "Nueva Descripción Plan de Prueba",
        "status": 1,
        "image": "URL_IMAGE",
        "metadata": {
        "DNI": 123456782
        }
    }"#;

    const REQUEST_TIPO_ORDER_BODY : &str = r#"{
        "id": "ord_test_xjmEW4dIyJM9G4cc",
        "order_types": [
        "cuotealo",
        "cip"]
    }"#;

    const REQUEST_SUSCRIPCION_UPDATE_BODY : &str = r#"{
        "card_id": "crd_test_XXXXXXXXXXXXXXXX",
        "metadata": { }
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
    async fn test_cargo_get() {

        match get("charges", "chr_test_7VUwCneoG1XtLeS7", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_cargo_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_CHARGE_UPDATE_BODY, "charges/chr_test_7VUwCneoG1XtLeS7", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_charge_capture() {
        // Ejemplo de cómo usar la función
        match create("", "charges/chr_test_7VUwCneoG1XtLeS7/capture", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_order_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_ORDER_BODY, "orders", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }


    #[tokio::test]
    async fn test_order_confirm() {
        // Ejemplo de cómo usar la función

        match create("", "orders/ord_test_0CjjdWhFpEAZlxlz/confirm", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_order_type_confirm() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_TIPO_ORDER_BODY, "orders/confirm", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_order_get() {

        match get("orders", "ord_test_QDO81GT6Zaseewkp", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_order_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_ORDER_UPDATE_BODY, "orders/ord_test_QDO81GT6Zaseewkp", PUBLIC_KEY, SECRET_KEY).await {
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

        match delete("orders", "ord_test_20HjpSkdDlSdoHEC", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_refund_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_REFUND_BODY, "refunds", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_refund_get() {

        match get("refunds", "ref_test_7lYOtOMM6LxcgJUW", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_refund_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_REFUND_UPDATE_BODY, "refunds/ref_test_7lYOtOMM6LxcgJUW", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_client_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_CLIENT_BODY, "customers", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_client_get() {

        match get("customers", "cus_test_QDO81GT6Zaseewkp", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_client_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_REFUND_UPDATE_BODY, "customers/cus_test_QDO81GT6Zaseewkp", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_client_delete() {

        match delete("customers", "cus_test_QDO81GT6Zaseewkp", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_card_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_CARD_BODY, "cards", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_card_get() {

        match get("cards", "crd_test_QDO81GT6Zaseewkp", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_card_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_REFUND_UPDATE_BODY, "cards/crd_test_QDO81GT6Zaseewkp", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_card_delete() {

        match delete("cards", "crd_test_QDO81GT6Zaseewkp", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_PLAN_BODY, "recurrent/plans/create", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_plan_get() {

        match get("recurrent/plans", "pln_test_XXXXXXXXXXXXXXXX", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_PLAN_UPDATE_BODY, "recurrent/plans/pln_test_XXXXXXXXXXXXXXXX", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_plan_delete() {

        match delete("recurrent/plans", "pln_test_XXXXXXXXXXXXXXXX", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_subscriptions_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_SUSCRIPCION_BODY, "current/subscriptions/create", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_subscriptions_get() {

        match get("recurrent/subscriptions", "sxn_test_XXXXXXXXXXXXXXXXXXX", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_subscriptions_update() {
        // Ejemplo de cómo usar la función

        match update(REQUEST_SUSCRIPCION_UPDATE_BODY, "recurrent/subscriptions/sxn_test_XXXXXXXXXXXXXXXXXXX", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_subscriptions_delete() {

        match delete("recurrent/subscriptions", "sxn_test_XXXXXXXXXXXXXXXXXXX", SECRET_KEY).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

}