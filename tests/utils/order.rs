use LibCulqi::culqi::order::Order;

use super::util::{assert_status, create_client, parse_response_body};
use crate::request::order::create_order_request;

#[allow(dead_code)]
pub async fn create_order() -> String {
    match Order::create(&create_client(), &create_order_request(), None,).await {
        Ok(response,) => {
            assert_status(&response, 201,);
            let response_json = parse_response_body(response,).await;
            response_json["id"]
                .as_str()
                .expect("El campo 'id' no es una cadena",)
                .to_string()
        }
        Err(rejection,) => {
            println!("Error al crear la orden: {:?}", rejection);
            panic!("La prueba falló debido a un error al crear la orden");
        }
    }
}
