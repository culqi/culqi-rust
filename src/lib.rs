use std::error::Error;
use reqwest::{Client, RequestBuilder};
mod encrypt;
use encrypt::encrypt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct ResponseData {
    pub text: String,
    pub status_code: u16,
}

pub async fn create(
    body: &str,
    action: &str,
    pk: &str,
    sk: &str,
) -> Result<(String, u16), reqwest::Error> {
    let SKEY: &str = sk;
    let PKEY: &str = pk;
    const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
    const BASE_URL: &str = "https://api.culqi.com/v2/";

    let key: &str;
    let url: String;

    if action == "tokens" {
        key = PKEY;
        url = SECURE_URL.to_string();
    } else {
        key = SKEY;
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


pub async fn createEncrypt(
    body: &str,
    action: &str,
    pkey: &str,
    skey: &str,
    rsa_key: &str,
    rsa_id: &str,
) -> Result<(String, u16), Box<dyn Error>> {
    const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
    const BASE_URL: &str = "https://api.culqi.com/v2/";

    let body_encrypt = encrypt(body, rsa_key, true)?;

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
     //"x-culqi-rsa-id" => rsa_id
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, ("Bearer ".to_owned() + key).parse().unwrap());
    headers.insert("x-culqi-rsa-id", (rsa_id).parse().unwrap());

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
    const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
    const BASE_URL: &str = "https://api.culqi.com/v2/";

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

    const SECURE_URL: &str = "https://secure.culqi.com/v2/tokens";
    const BASE_URL: &str = "https://api.culqi.com/v2/";

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_token() {
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

        let sk = "sk_test_1573b0e8079863ff";
        let pk = "pk_test_90667d0a57d45c48";

        match create(body, "tokens", pk, sk).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
    #[tokio::test]
    async fn test_create() {
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

        let sk = "sk_test_1573b0e8079863ff";
        let pk = "pk_test_90667d0a57d45c48";

        match create(body, "charges", pk, sk).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_tokenEncrypt() {
        // Ejemplo de cómo usar la función
        let body = "{\"card_number\":\"4111111111111111\",\"cvv\":\"123\",\"expiration_month\":\"09\",\"expiration_year\":\"2025\",\"email\":\"alexis.pumayalla@culqi.com\",\"metadata\":{\"coment\":\"Tarjeta de prueba alexis\"}}";
        let sk = "sk_test_1573b0e8079863ff";
        let pk = "pk_test_90667d0a57d45c48";
        let rsa_key = "-----BEGIN PUBLIC KEY-----
        MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDYp0451xITpczkBrl5Goxkh7m1
        oynj8eDHypIn7HmbyoNJd8cS4OsT850hIDBwYmFuwmxF1YAJS8Cd2nes7fjCHh+7
        oNqgNKxM2P2NLaeo4Uz6n9Lu4KKSxTiIT7BHiSryC0+Dic91XLH7ZTzrfryxigsc
        +ZNndv0fQLOW2i6OhwIDAQAB
        -----END PUBLIC KEY-----";
        let rsa_id = "508fc232-0a9d-4fc0-a192-364a0b782b89";

        match createEncrypt(body, "tokens", pk, sk, rsa_key, rsa_id).await  {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_token_get() {

        let sk = "sk_test_1573b0e8079863ff";

        match get("tokens", "tkn_test_20HjpSkdDlSdoHEC", sk).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    #[tokio::test]
    async fn test_order_delete() {

        let sk = "sk_test_1573b0e8079863ff";

        match delete("tokens", "ord_test_20HjpSkdDlSdoHEC", sk).await {
            Ok(response_text) => println!("Respuesta del servidor: {}", response_text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
