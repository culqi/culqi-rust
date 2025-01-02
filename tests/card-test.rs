use BrandoCulqi::culqi::card::Card;
mod config;
mod header;
mod request;
mod utils;

#[cfg(test)]
mod card_tests {
    use header::header_rsa;
    use request::card::{create_card_request, request_card_all, update_card_request};
    use serial_test::serial;
    use utils::{card, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_card_create() {
        println!("Crear Card -> ");
        match Card::create(&util::create_client(), &create_card_request().await, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'card' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el card: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el card");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_card_create_encrypt() {
        println!("Crear Card con llaves RSA-> ");
        match Card::create(
            &util::create_client_encrypt(),
            &create_card_request().await,
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'card' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el card: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el card");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_card_update() {
        println!("Crear Card -> ");
        let card_id = card::create_card().await;
        println!("Actualizar Card -> ");
        match Card::patch(
            &util::create_client(),
            &card_id,
            &update_card_request().await,
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'card' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el card: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el card");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_card_get() {
        println!("Crear Card -> ");
        let card_id = card::create_card().await;
        println!("Obtener Card por Id {:?} -> ", card_id);
        match Card::get(&util::create_client(), &card_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["object"].is_string(),
                    "El campo 'card' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener card por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener card por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_card_all() {
        println!("Listar Card -> ");
        match Card::all(&util::create_client(), &request_card_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar card: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar card");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_card_delete() {
        println!("Listar Card -> ");
        let card_id = card::create_card().await;
        println!("Eliminar Card por Id {:?} -> ", card_id);
        match Card::delete(&util::create_client(), &card_id, None,).await {
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
                println!("Error al eliminar el card: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el card");
            }
        }
    }
}
