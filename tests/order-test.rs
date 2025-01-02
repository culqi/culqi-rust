use LibCulqi::culqi::order::Order;
mod config;
mod header;
mod request;
mod utils;
use request::order::{create_order_request, update_order_request};

#[cfg(test)]
mod oder_tests {
    use header::header_rsa;
    use request::order::{order_type_confirm_request, request_order_all};
    use serial_test::serial;
    use utils::{order, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_order_create() {
        println!("Crear Orden -> ");
        match Order::create(&util::create_client(), &create_order_request(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear la orden: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear la orden");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_create_encrypt() {
        println!("Crear Orden con llaves RSA-> ");
        match Order::create(
            &util::create_client_encrypt(),
            &create_order_request(),
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear la orden: {:?} encriptada", rejection);
                panic!("La prueba falló debido a un error al crear la orden encriptada");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_get() {
        println!("Crear Orden -> ");
        let order_id = order::create_order().await;
        println!("Obtener Orden por Id {:?} -> ", order_id);
        match Order::get(&util::create_client(), &order_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener orden por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener orden por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_list() {
        println!("Listar Orden -> ");
        match Order::all(&util::create_client(), &request_order_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar orden: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar orden");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_delete() {
        println!("Crear Orden -> ");
        let order_id = order::create_order().await;
        println!("Eliminar Orden por Id {:?} -> ", order_id);
        match Order::delete(&util::create_client(), &order_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 204,);
            }
            Err(rejection,) => {
                println!("Error al eliminar la orden: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar la orden");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_update() {
        println!("Crear Orden -> ");
        let order_id = order::create_order().await;
        println!("Actualizar Orden por Id {:?} -> ", order_id);
        match Order::patch(
            &util::create_client(),
            &order_id,
            &update_order_request(),
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar la orden: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar la orden");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_confirm() {
        println!("Crear Orden -> ");
        let order_id = order::create_order().await;
        println!("Confirmar Orden por Id {:?} -> ", order_id);
        match Order::confirm(&util::create_client(), &order_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar la orden: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar la orden");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_order_confirm_type() {
        println!("Crear Orden -> ");
        let order_id = order::create_order().await;
        println!("Confirmar Tipo de Orden por Id {:?} -> ", order_id);
        match Order::type_confirm(
            &util::create_client(),
            &order_type_confirm_request(&order_id,),
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "order");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar la orden: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar la orden");
            }
        }
    }
}
