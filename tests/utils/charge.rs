use BrandoCulqi::culqi::charge::Charge;

use super::util::{assert_status, create_client, parse_response_body};
use crate::request::charge::create_charge_request;

#[allow(dead_code)]
pub async fn create_charge() -> String {
    match Charge::create(&create_client(), &create_charge_request().await, None,).await {
        Ok(response,) => {
            print!("Response Charge: {:?}", response);
            assert_status(&response, 201,);
            let response_json = parse_response_body(response,).await;
            response_json["id"]
                .as_str()
                .expect("El campo 'id' no es una cadena",)
                .to_string()
        }
        Err(rejection,) => {
            println!("Error al crear el charge: {:?}", rejection);
            panic!("La prueba falló debido a un error al crear el charge");
        }
    }
}
