use BrandoCulqi::culqi::refund::Refund;
mod config;
mod header;
mod request;
mod utils;

#[cfg(test)]
mod refund_tests {
    use header::header_rsa;
    use request::refund::{create_refund_request, request_refund_all, update_refund_request};
    use serial_test::serial;
    use utils::{refund, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_refund_create() {
        println!("Crear Refund -> ");
        match Refund::create(&util::create_client(), &create_refund_request().await, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'refund' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el refund: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el refund");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_refund_create_encrypt() {
        println!("Crear Refund con llaves RSA-> ");
        match Refund::create(
            &util::create_client_encrypt(),
            &create_refund_request().await,
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'refund' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el refund: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el refund");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_refund_update() {
        println!("Crear Refund -> ");
        let refund_id = refund::create_refund().await;
        println!("Actualizar Refund -> ");
        match Refund::patch(
            &util::create_client(),
            &refund_id,
            &update_refund_request().await,
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'refund' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el refund: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el refund");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_refund_get() {
        println!("Crear Refund -> ");
        let refund_id = refund::create_refund().await;
        println!("Obtener Refund por Id {:?} -> ", refund_id);
        match Refund::get(&util::create_client(), &refund_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'refund' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener refund por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener refund por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_refund_all() {
        println!("Listar Refund -> ");
        match Refund::all(&util::create_client(), &request_refund_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar refund: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar refund");
            }
        }
    }
}
