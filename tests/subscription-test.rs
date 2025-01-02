use LibCulqi::culqi::subscription::Subscription;
mod config;
mod header;
mod request;
mod utils;

#[cfg(test)]
mod subscription_tests {
    use header::header_rsa;
    use request::subscription::{
        create_subscription_request, request_subscription_all, update_subscription_request,
    };
    use serial_test::serial;
    use utils::{subscription, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_subscription_create() {
        println!("Crear Subscription -> ");
        match Subscription::create(
            &util::create_client(),
            &create_subscription_request().await,
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el subscription: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear la subscription");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_subscription_create_encrypt() {
        println!("Crear Subscription con llaves RSA -> ");
        match Subscription::create(
            &util::create_client_encrypt(),
            &create_subscription_request().await,
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el subscription: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el subscription");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_subscription_update() {
        println!("Crear Subscription -> ");
        let subscription_id = subscription::create_subscription().await;
        println!("Actualizar Subscription -> ");
        match Subscription::patch(
            &util::create_client(),
            &subscription_id,
            &update_subscription_request().await,
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el subscription: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el subscription");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_subscription_get() {
        println!("Crear Subscription -> ");
        let subscription_id = subscription::create_subscription().await;
        println!("Obtener Subscription por Id {:?} -> ", subscription_id);
        match Subscription::get(&util::create_client(), &subscription_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener subscription por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener subscription por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_subscription_all() {
        println!("Listar Subscription -> ");
        match Subscription::all(&util::create_client(), &request_subscription_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar subscription: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar subscription");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_subscription_delete() {
        println!("Listar Subscription -> ");
        let subscription_id = subscription::create_subscription().await;
        println!("Eliminar Subscription por Id {:?} -> ", subscription_id);
        match Subscription::delete(&util::create_client(), &subscription_id, None,).await {
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
                println!("Error al eliminar el subscription: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el subscription");
            }
        }
    }
}
