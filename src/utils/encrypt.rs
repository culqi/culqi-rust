use std::{collections::HashMap, error::Error, fmt, io::Error as IoError, string::FromUtf8Error};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};
use base64::{engine::general_purpose, DecodeError, Engine};
use openssl::{
    encrypt::Encrypter,
    hash::MessageDigest,
    pkey::PKey,
    rsa::{Padding, Rsa},
};
use rand::Rng;
use serde_json::Value;

#[derive(Debug,)]
pub enum MyError {
    AesGcmError(aes_gcm::Error,),
    RsaError(openssl::error::ErrorStack,),
    JsonError(serde_json::Error,),
    Other(String,),
    IoError(IoError,),
    Decode(DecodeError,),
    Utf8Error(FromUtf8Error,),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match *self {
            MyError::AesGcmError(ref err,) => write!(f, "AES GCM error: {}", err),
            MyError::RsaError(ref err,) => write!(f, "RSA error: {}", err),
            MyError::JsonError(ref err,) => write!(f, "JSON error: {}", err),
            MyError::Other(ref err,) => write!(f, "Other error: {}", err),
            MyError::IoError(ref err,) => write!(f, "IO error: {}", err),
            MyError::Decode(ref err,) => write!(f, "IO error: {}", err),
            MyError::Utf8Error(ref err,) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for MyError {
    fn source(&self,) -> Option<&(dyn Error + 'static),> {
        match *self {
            MyError::AesGcmError(ref _err,) => None,
            MyError::RsaError(ref err,) => Some(err,),
            MyError::JsonError(ref err,) => Some(err,), // And this line
            MyError::Other(ref _err,) => None,
            MyError::IoError(ref err,) => Some(err,),
            MyError::Decode(ref err,) => Some(err,),
            MyError::Utf8Error(ref err,) => Some(err,),
        }
    }
}
impl From<String,> for MyError {
    fn from(error: String,) -> Self {
        MyError::Other(error,)
    }
}

impl From<FromUtf8Error,> for MyError {
    fn from(error: FromUtf8Error,) -> Self {
        MyError::Utf8Error(error,)
    }
}
impl From<IoError,> for MyError {
    fn from(err: IoError,) -> MyError {
        MyError::IoError(err,)
    }
}
impl From<aes_gcm::Error,> for MyError {
    fn from(err: aes_gcm::Error,) -> MyError {
        MyError::AesGcmError(err,)
    }
}

impl From<openssl::error::ErrorStack,> for MyError {
    fn from(err: openssl::error::ErrorStack,) -> MyError {
        MyError::RsaError(err,)
    }
}

// Add this implementation
impl From<serde_json::Error,> for MyError {
    fn from(err: serde_json::Error,) -> MyError {
        MyError::JsonError(err,)
    }
}

impl From<DecodeError,> for MyError {
    fn from(error: DecodeError,) -> Self {
        MyError::Decode(error,)
    }
}

pub fn encrypt(
    data: &str,
    rsa_public_key: &str,
    is_json: bool,
) -> Result<HashMap<String, String,>, MyError,> {
    println!("Initializing Encrypter...");

    let json_data: Value = if is_json {
        serde_json::from_str(data,)?
    } else {
        data.into()
    };
    let key: [u8; 32] = rand::thread_rng().r#gen();
    let iv: [u8; 12] = rand::thread_rng().r#gen();

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key,),);
    let nonce = GenericArray::from_slice(&iv,);

    let plaintext = serde_json::to_string(&json_data,)?.as_bytes().to_vec();
    let mut ciphertext = cipher.encrypt(nonce, &plaintext[..],)?;
    ciphertext = Vec::from(&ciphertext[..ciphertext.len() - 16],);

    let encrypted_data = general_purpose::STANDARD.encode(&ciphertext,);

    let public_key = Rsa::public_key_from_pem(rsa_public_key.as_bytes(),).unwrap();
    let public_key = PKey::from_rsa(public_key,).unwrap();

    let mut encrypter = Encrypter::new(&public_key,).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1_OAEP,).unwrap();
    encrypter.set_rsa_oaep_md(MessageDigest::sha256(),).unwrap();

    let buffer_len = encrypter.encrypt_len(&key,).unwrap();
    let mut encrypted_key = vec![0; buffer_len];

    let encrypted_len = encrypter.encrypt(&key, &mut encrypted_key,).unwrap();
    encrypted_key.truncate(encrypted_len,);

    let buffer_len = encrypter.encrypt_len(&iv,).unwrap();
    let mut encrypted_iv = vec![0; buffer_len];

    let encrypted_len = encrypter.encrypt(&iv, &mut encrypted_iv,).unwrap();
    encrypted_iv.truncate(encrypted_len,);

    let encrypted_key_b64 = general_purpose::STANDARD.encode(&encrypted_key,);
    let encrypted_iv_b64 = general_purpose::STANDARD.encode(&encrypted_iv,);

    let mut map = HashMap::new();
    map.insert("encrypted_data".to_string(), encrypted_data,);
    map.insert("encrypted_key".to_string(), encrypted_key_b64,);
    map.insert("encrypted_iv".to_string(), encrypted_iv_b64,);

    Ok(map,)
}
