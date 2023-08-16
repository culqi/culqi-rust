use isahc::prelude::*;
use std::convert::TryInto;
use std::error::Error;
use aes_gcm::aead::{Aead};
use base64::{DecodeError};
use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error;
use rsa::{PublicKey};
use crate::Encrypt;
use crate::Encrypt::encrypt;


const skey : &'static str = "sk_live_34a07dcb6d4c7e39";
const pkey : &'static str = "pk_live_889113cd74ecfc55";
const rsaid : &'static str = "508fc232-0a9d-4fc0-a192-364a0b782b89";
const CULQI_RSA_KEY: &'static str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDYp0451xITpczkBrl5Goxkh7m1
oynj8eDHypIn7HmbyoNJd8cS4OsT850hIDBwYmFuwmxF1YAJS8Cd2nes7fjCHh+7
oNqgNKxM2P2NLaeo4Uz6n9Lu4KKSxTiIT7BHiSryC0+Dic91XLH7ZTzrfryxigsc
+ZNndv0fQLOW2i6OhwIDAQAB
-----END PUBLIC KEY-----";

const SECUREURL: &'static str = "https://qa-secure.culqi.xyz/v2/tokens";

const BASEURL: &'static str = "https://qa-api.culqi.xyz/v2/";


#[derive(Debug)]
pub enum MyError {
    AesGcmError(aes_gcm::Error),
    RsaError(openssl::error::ErrorStack),
    JsonError(serde_json::Error),  // Add this line
    Other(String),
    IoError(IoError),
    Decode(DecodeError),
    Utf8Error(FromUtf8Error),
    EncryptionError(String),

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
            MyError::EncryptionError(ref err) => write!(f, "Encryption error: {}", err),

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
            MyError::EncryptionError(ref _err) => None,
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


impl From<Encrypt::MyError> for MyError {
    fn from(err: Encrypt::MyError) -> MyError {
        // Aquí necesitas convertir de Encrypt::MyError a MyError
        // Esto dependerá de cómo estén definidos los errores y cuál sea
        // la lógica adecuada para la conversión.
        // Por ejemplo:
        MyError::EncryptionError(err.to_string()) // Suponiendo que MyError tiene una variante EncryptionError
    }
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

pub fn create(
    body: &str,
    action : &str
) -> Result<(String, u16), isahc::Error> {



    let key: &str;
    let url: String;

    if action == "tokens" {
        key = pkey;
        url = SECUREURL.to_string();
    } else {
        key = skey;
        url = BASEURL.to_owned() + action;
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

pub fn update(
    action : &str,
    query: &str,
    body: &str
) -> Result<String, isahc::Error> {



    let key: &str;
    let url: String;

    key = skey;
    url = BASEURL.to_owned() + action + "/" + query;

    let request = isahc::Request::patch(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + key)
        .body(body)?
        send()?;

    let mut response = isahc::send(request)?;
    let response_text = response.text()?;
    Ok(response_text)
}