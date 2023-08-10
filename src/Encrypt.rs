use std::convert::TryInto;
use std::error::Error;
use openssl::rsa::{Padding, Rsa};
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use rand::Rng;
use openssl::pkey::PKey;
use base64::{DecodeError, encode};
use std::collections::HashMap;
use std::fmt;
use serde_json::{Value};
use std::io::Error as IoError;
use std::string::FromUtf8Error;
use rsa::{PublicKey};
use openssl::hash::MessageDigest;
use openssl::encrypt::{Encrypter};

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