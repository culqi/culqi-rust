use std::error::Error;
use reqwest::{Client, RequestBuilder};

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

pub fn createEncrypt(
    body: &str,
    action : &str
) -> Result<(String, u16), MyError> {

    let body_encrypt = match encrypt(body, CULQI_RSA_KEY, true) {
        Ok(result) => result,
        Err(err) => return Err(MyError::from(err)), // Aquí simplemente retornamos el error original
    };

    println!("body_encrypt: {:?}", body_encrypt);


    let key: &str;
    let url: String;

    println!("action: {:?}", action);

    if action == "tokens" {
        key = pkey;
        url = SECUREURL.to_string();
    } else {
        key = skey;
        url = BASEURL.to_owned() + action;
    }
    println!("key: {:?}", key);
    let mut response = isahc::Request::post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + key)
        .header("x-culqi-rsa-id", rsaid)
        //.headers(headers)
        .body(serde_json::to_string(&body_encrypt).unwrap())?
        .send()?;


    let status_code = response.status().as_u16();
    let response_text = response.text()?;

    Ok((response_text, status_code))
}



pub fn get(
    action : &str,
    query: &str
) -> Result<String, isahc::Error> {



    let key: &str;
    let url: String;

    key = skey;
    url = BASEURL.to_owned() + action + "/" + query;

    let request = isahc::Request::get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + key)
        .body(()) // No se envía un cuerpo en una solicitud GET
        .unwrap();

    let mut response = isahc::send(request)?;
    let response_text = response.text()?;
    Ok(response_text)
}

pub fn delete(
    action : &str,
    query: &str
) -> Result<String, isahc::Error> {

    let key: &str;
    let url: String;

    key = skey;
    url = BASEURL.to_owned() + action + "/" + query;

    let request = isahc::Request::delete(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + key)
        .body(()) // No se envía un cuerpo en una solicitud GET
        .unwrap();

    let mut response = isahc::send(request)?;
    let response_text = response.text()?;
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

        match create(body, "tokens").await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
