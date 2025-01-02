use LibCulqi::culqi::charge::Charge;
mod config;
mod header;
mod request;
mod utils;

#[cfg(test)]
mod charge_tests {
    use header::{charge::get_header_charge_recurrent, header_rsa};
    use request::charge::{create_charge_request, request_charge_all, update_charge_request};
    use serial_test::serial;
    use utils::{charge, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_charge_create() {
        println!("Crear Charge -> ");
        match Charge::create(&util::create_client(), &create_charge_request().await, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'charge' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el charge: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el charge");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_charge_recurrent_create() {
        println!("Crear Charge -> ");
        match Charge::create(
            &util::create_client(),
            &create_charge_request().await,
            Some(get_header_charge_recurrent(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'charge' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el charge: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el charge");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_charge_create_encrypt() {
        println!("Crear Charge con llaves RSA-> ");
        match Charge::create(
            &util::create_client_encrypt(),
            &create_charge_request().await,
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'charge' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el charge: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el charge");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_charge_update() {
        println!("Crear Charge -> ");
        let charge_id = charge::create_charge().await;
        println!("Actualizar Charge -> ");
        match Charge::patch(
            &util::create_client(),
            &charge_id,
            &update_charge_request().await,
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'charge' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el charge: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el charge");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_charge_get() {
        println!("Crear Charge -> ");
        let charge_id = charge::create_charge().await;
        println!("Obtener Charge por Id {:?} -> ", charge_id);
        match Charge::get(&util::create_client(), &charge_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'charge' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener charge por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener charge por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_charge_all() {
        println!("Listar Charge -> ");
        match Charge::all(&util::create_client(), &request_charge_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar charge: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar charge");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_charge_capture() {
        println!("Listar Charge -> ");
        let charge_id = charge::create_charge().await;
        println!("Capturar Charge por Id {:?} -> ", charge_id);
        match Charge::capture(&util::create_client(), &charge_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al capturar el charge: {:?}", rejection);
                panic!("La prueba falló debido a un error al capturar el charge");
            }
        }
    }
}
