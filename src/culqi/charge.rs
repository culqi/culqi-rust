extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Charge {
}

impl Charge {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let ref antifraud_details = culqi::AntifraudDetails::new("av. lima", "lima", "PE", "Will", "Aguirre", "993978969");
    //! ```
    //!
    //! ```
    //! let new_charge = culqi::Charge::new("3500", "PEN", "will@me.com", charge.installments, None, Some(antifraud_details), "{TOKEN}");
    //! ```
    //!
    //! ```
    //! let create_charge = culqi::Charge::create(&client, &new_charge);
    //! ```
    //!
    //! ```
    //! let get_charge = culqi::Charge::retrieve(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! let capture_charge = culqi::Charge::capture(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut charge_filter: HashMap<String, String>;
    //! charge_filter = HashMap::new();
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! charge_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! charge_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! charge_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_charges = culqi::Charge::all(&client_sk, Some(charge_filter), None, None, None);
    //! ```
    //!

    pub fn new<S: Into<String>>(
        amount: S,
        currency_code: S,
        email: S,
        installments: i32,
        metadata: Option<&HashMap<String, serde_json::Value>>,
        antifraud_details: Option<&HashMap<String, serde_json::Value>>,
        source_id: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("amount".to_string(), json!(amount.into()));
        map.insert("currency_code".to_string(), json!(currency_code.into()));
        map.insert("email".to_string(), json!(email.into()));
        map.insert("installments".to_string(), json!(installments));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        if !antifraud_details.is_none() {
            map.insert("antifraud_details".to_string(), json!(antifraud_details));
        }
        map.insert("source_id".to_string(), json!(source_id.into()));
        return map;
    }

    pub fn create(client: &Client, charge: &HashMap<String, serde_json::Value>) -> String {
        client.post("/charges", charge)
    }

    pub fn capture(client: &Client, id: &str) -> String {
        client.capture(&format!("/charges/{}/capture", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/charges/{}", id))
    }

    pub fn all(
        client: &Client,
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/charges", query_params, limit, before, after)
    }

}
