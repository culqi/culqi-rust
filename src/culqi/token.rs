use std::error::Error;
//use reqwest::{Client, RequestBuilder};
//
use crate::client::Client;
//use crate::client::Client::create;
use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
//use serde_json::json;



pub async fn create(
    body: &str,
    action: &str,
    pkey: &str,
    skey: &str,
    rsa_pid: &str,
    rsa_pkey: &str,
) -> Result<(String, u16), Box<dyn Error>> {

    create(body, action, pkey, skey, rsa_pid, rsa_pkey);

    //let status_code = response.status().as_u16();
    //let response_text = response.text().await?;

    //Ok((response_text, status_code))
    Ok(("".to_string(), 201))
}
