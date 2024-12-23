use BrandoCulqi::culqi::customer::Customer;

use super::util::{assert_status, create_client, parse_response_body};
use crate::request::customer::create_customer_request;

#[allow(dead_code)]
pub async fn create_customer() -> String {
    match Customer::create(&create_client(), &create_customer_request(), None,).await {
        Ok(response,) => {
            assert_status(&response, 201,);
            let response_json = parse_response_body(response,).await;
            response_json["id"]
                .as_str()
                .expect("El campo 'id' no es una cadena",)
                .to_string()
        }
        Err(rejection,) => {
            println!("Error al crear el customer: {:?}", rejection);
            panic!("La prueba falló debido a un error al crear el customer");
        }
    }
}
