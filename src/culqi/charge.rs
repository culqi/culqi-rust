extern crate serde_json;

use client::Client;

#[derive(Debug, Serialize)]
pub struct Charge {
    pub amount: String,
    pub currency_code: String,
    pub email: String,
    pub source_id: String
}

impl Charge {

    pub fn new<S: Into<String>>(
        amount: S,
        currency_code: S,
        email: S,
        source_id: S
    ) -> Charge {
        Charge {
            amount: amount.into(),
            currency_code: currency_code.into(),
            email: email.into(),
            source_id: source_id.into()
        }
    }

    pub fn create(client: &Client, charge: &Charge) -> String {
        let mut charge_json = serde_json::to_string(charge);
        client.post("/charges", charge_json.unwrap().as_str())
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/charges/{}", id))
    }

}
