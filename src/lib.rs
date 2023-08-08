use isahc::prelude::*;
use std::convert::TryInto;
use std::error::Error;
use std::io::Read;
use rsa::pkcs8::der::Decodable;
use rsa::pkcs1::FromRsaPublicKey;
use openssl::rsa::{Padding, Rsa};
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use rand::Rng;
use openssl::pkey::PKey;
use openssl::symm::Cipher;
use base64::{decode, DecodeError, encode};
use std::collections::HashMap;
use std::fmt;
use serde_json::{json, Value};
use std::io::Error as IoError;
use regex::Regex;
use std::string::FromUtf8Error;
use rsa::{PublicKey, PaddingScheme};
use openssl::hash::MessageDigest;
use rand::rngs::OsRng;
use openssl::encrypt::{Encrypter, Decrypter};



#[derive(Debug)]
pub enum MyError {
    AesGcmError(aes_gcm::Error),
    RsaError(openssl::error::ErrorStack),
    JsonError(serde_json::Error),  // Add this line
    Other(String),
    IoError(IoError),
    Decode(DecodeError),
    Utf8Error(FromUtf8Error),

}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::AesGcmError(ref err) => write!(f, "AES GCM error: {}", err),
            MyError::RsaError(ref err) => write!(f, "RSA error: {}", err),
            MyError::JsonError(ref err) => write!(f, "JSON error: {}", err),  // And this line
            MyError::Other(ref err) => write!(f, "Other error: {}", err),
            MyError::IoError(ref err) => write!(f, "IO error: {}", err),
            MyError::Decode(ref err) => write!(f, "IO error: {}", err),
            MyError::Utf8Error(ref err) => write!(f, "IO error: {}", err),

        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MyError::AesGcmError(ref _err) => None,
            MyError::RsaError(ref err) => Some(err),
            MyError::JsonError(ref err) => Some(err),  // And this line
            MyError::Other(ref _err) => None,
            MyError::IoError(ref err) => Some(err),
            MyError::Decode(ref err) => Some(err),
            MyError::Utf8Error(ref err) => Some(err),
        }
    }
}
impl From<FromUtf8Error> for MyError {
    fn from(error: FromUtf8Error) -> Self {
        // Aquí puedes definir cómo convertir el error `FromUtf8Error` en tu error personalizado
        // Esto dependerá de cómo hayas definido `MyError`
        MyError::Utf8Error(error) // Por ejemplo, si agregas un variant `Utf8Error` en tu enum `MyError`
    }
}
impl From<IoError> for MyError {
    fn from(err: IoError) -> MyError {
        MyError::IoError(err)
    }
}
impl From<aes_gcm::Error> for MyError {
    fn from(err: aes_gcm::Error) -> MyError {
        MyError::AesGcmError(err)
    }
}

impl From<openssl::error::ErrorStack> for MyError {
    fn from(err: openssl::error::ErrorStack) -> MyError {
        MyError::RsaError(err)
    }
}

impl From<isahc::Error> for MyError {
    fn from(err: isahc::Error) -> MyError {
        MyError::Other(err.to_string())
    }
}

// Add this implementation
impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::JsonError(err)
    }
}

impl From<isahc::http::Error> for MyError {
    fn from(err: isahc::http::Error) -> MyError {
        MyError::Other(err.to_string())
    }
}

impl From<DecodeError> for MyError {
    fn from(error: DecodeError) -> Self {
        // Aquí puedes definir cómo convertir el error `DecodeError` en tu error personalizado
        // Esto dependerá de cómo hayas definido `MyError`
        MyError::Decode(error) // Por ejemplo, si tienes un variant `Decode` en tu enum `MyError`
    }
}

pub fn encrypt(data: &str, publicKey: &str, is_json: bool) -> Result<HashMap<String, String>, MyError> {


    let json_data: Value = if is_json {
        serde_json::from_str(&data)?
    } else {
        data.into() // or handle non-JSON data as needed
    };
    let json_string = serde_json::to_string(&json_data)?;
    println!("JSON data: {}", json_string);

    // Generate a 256-bit random key for AES encryption
    let key: [u8; 32] = rand::thread_rng().gen();

    // GCM mode requires a 96-bit (12 bytes) random initialization vector
    let iv: [u8; 12] = rand::thread_rng().gen();

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
    let nonce = GenericArray::from_slice(&iv);

    // The data to be encrypted
    let mut plaintext = serde_json::to_string(&json_data)?.as_bytes().to_vec();

    // Perform encryption
    let mut ciphertext = cipher.encrypt(nonce, &plaintext[..])?;
    ciphertext = Vec::from(&ciphertext[..ciphertext.len() - 16]);


    // Convert encrypted data to base64 string
    let encrypted_data = encode(&ciphertext);



    let public_key = Rsa::public_key_from_pem(publicKey.as_bytes()).unwrap();
    let public_key = PKey::from_rsa(public_key).unwrap();


    // Encrypt message
    let mut encrypter = Encrypter::new(&public_key).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1_OAEP).unwrap();
    encrypter.set_rsa_oaep_md(MessageDigest::sha256()).unwrap();

    // Create an output buffer
    let buffer_len = encrypter.encrypt_len(&key).unwrap();
    let mut encrypted_key = vec![0; buffer_len];

    // Encrypt and truncate buffer
    let encrypted_len = encrypter.encrypt(&key, &mut encrypted_key).unwrap();
    encrypted_key.truncate(encrypted_len);

    // Create an output buffer
    let buffer_len = encrypter.encrypt_len(&iv).unwrap();
    let mut encrypted_iv = vec![0; buffer_len];

    // Encrypt and truncate buffer
    let encrypted_len = encrypter.encrypt(&iv, &mut encrypted_iv).unwrap();
    encrypted_iv.truncate(encrypted_len);

    let encrypted_key_b64 = encode(&encrypted_key);
    let encrypted_iv_b64 = encode(&encrypted_iv);


    let mut map = HashMap::new();
    map.insert("encrypted_data".to_string(), encrypted_data);
    map.insert("encrypted_key".to_string(), encrypted_key_b64);
    map.insert("encrypted_iv".to_string(), encrypted_iv_b64);

    Ok(map)
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct ResponseData {
    pub text: String,
    pub status_code: u16,
}
pub fn createEncrypt(
    body: &str,
    action : &str
) -> Result<(String, u16), MyError> {
    let skey = "sk_live_34a07dcb6d4c7e39";
    let pkey = "pk_live_889113cd74ecfc55";
    let rsaid = "508fc232-0a9d-4fc0-a192-364a0b782b89";
    const CULQI_RSA_KEY: &'static str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDYp0451xITpczkBrl5Goxkh7m1
oynj8eDHypIn7HmbyoNJd8cS4OsT850hIDBwYmFuwmxF1YAJS8Cd2nes7fjCHh+7
oNqgNKxM2P2NLaeo4Uz6n9Lu4KKSxTiIT7BHiSryC0+Dic91XLH7ZTzrfryxigsc
+ZNndv0fQLOW2i6OhwIDAQAB
-----END PUBLIC KEY-----";
    let body_encrypt = encrypt(body, CULQI_RSA_KEY, true)?;
    println!("body_encrypt: {:?}", body_encrypt);


    let key: &str;
    let url: String;

    println!("action: {:?}", action);

    if action == "tokens" {
        key = pkey;
        url = "https://qa-secure.culqi.xyz/v2/tokens".to_string();
    } else {
        key = skey;
        url = "https://qa-api.culqi.xyz/v2/".to_owned() + action;
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

pub fn create(
    body: &str,
    action : &str
) -> Result<(String, u16), isahc::Error> {
    let skey = "sk_test_1573b0e8079863ff";
    let pkey = "pk_test_90667d0a57d45c48";


    let key: &str;
    let url: String;

    if action == "tokens" {
        key = pkey;
        url = "https://secure.culqi.com/v2/tokens".to_string();
    } else {
        key = skey;
        url = "https://api.culqi.com/v2/".to_owned() + action;
    }

    let mut response = isahc::Request::post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + key)
        //.headers(headers)
        .body(body)?
        .send()?;


    let status_code = response.status().as_u16();
    let response_text = response.text()?;

    Ok((response_text, status_code))
}

pub fn get(
    action : &str,
    query: &str
) -> Result<String, isahc::Error> {

    let skey = "sk_test_1573b0e8079863ff";

    let key: &str;
    let url: String;

    key = skey;
    url = "https://api.culqi.com/v2/".to_owned() + action + "/" + query;

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

    let skey = "sk_test_1573b0e8079863ff";

    let key: &str;
    let url: String;

    key = skey;
    url = "https://api.culqi.com/v2/".to_owned() + action + "/" + query;

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
}
