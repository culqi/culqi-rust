use LibCulqi::culqi::card::Card;

use super::util::{assert_status, create_client, parse_response_body};
use crate::request::card::create_card_request;

#[allow(dead_code)]
pub async fn create_card() -> String {
    match Card::create(&create_client(), &create_card_request().await, None,).await {
        Ok(response,) => {
            assert_status(&response, 201,);
            let response_json = parse_response_body(response,).await;
            response_json["id"]
                .as_str()
                .expect("El campo 'id' no es una cadena",)
                .to_string()
        }
        Err(rejection,) => {
            println!("Error al crear el card: {:?}", rejection);
            panic!("La prueba falló debido a un error al crear el card");
        }
    }
}
