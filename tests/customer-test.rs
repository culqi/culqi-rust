use BrandoCulqi::culqi::customer::Customer;
mod config;
mod header;
mod request;
mod utils;

#[cfg(test)]
mod tests {
    use header::header_rsa;
    use request::customer::{
        create_customer_request, request_customer_all, update_customer_request,
    };
    use serial_test::serial;
    use utils::{customer, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_customer_create() {
        println!("Crear Customer -> ");
        match Customer::create(&util::create_client(), &create_customer_request(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'customer' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el customer: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el customer");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_customer_create_encrypt() {
        println!("Crear Customer con llaves RSA-> ");
        match Customer::create(
            &util::create_client_encrypt(),
            &create_customer_request(),
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'customer' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el customer: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el customer");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_customer_update() {
        println!("Crear Customer -> ");
        let customer_id = customer::create_customer().await;
        println!("Actualizar Customer -> ");
        match Customer::patch(
            &util::create_client(),
            &customer_id,
            &update_customer_request(),
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'customer' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el customer: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el customer");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_customer_get() {
        println!("Crear Customer -> ");
        let customer_id = customer::create_customer().await;
        println!("Obtener Customer por Id {:?} -> ", customer_id);
        match Customer::get(&util::create_client(), &customer_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'customer' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener customer por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener customer por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_customer_all() {
        println!("Listar Customer -> ");
        match Customer::all(&util::create_client(), &request_customer_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar customer: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar customer");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_customer_delete() {
        println!("Listar Customer -> ");
        let customer_id = customer::create_customer().await;
        println!("Eliminar Customer por Id {:?} -> ", customer_id);
        match Customer::delete(&util::create_client(), &customer_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert_eq!(response_json["deleted"], true);
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el customer: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el customer");
            }
        }
    }
}
