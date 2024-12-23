use BrandoCulqi::culqi::token::Token;
mod config;
mod header;
mod request;
mod utils;
use request::token::{create_token_request, create_token_yape_request, update_token_request};

#[cfg(test)]
mod tests {
    use header::header_rsa;
    use request::token::request_token_all;
    use serial_test::serial;
    use utils::{token, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_token_create() {
        println!("Crear Token -> ");
        match Token::create(&util::create_client(), &create_token_request(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el token: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear  el token");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_token_create_encrypt() {
        println!("Crear Token con llaves RSA-> ");
        match Token::create(
            &util::create_client_encrypt(),
            &create_token_request(),
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el token: {:?} encriptada", rejection);
                panic!("La prueba falló debido a un error al crear el token encriptada");
            }
        }
    }

    #[tokio::test]
    async fn test_token_get() {
        println!("Crear Token -> ");
        let token_id = token::create_token().await;
        println!("Obtener Token por Id {:?} -> ", token_id);
        match Token::get(&util::create_client(), &token_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener el token por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener el token por id");
            }
        }
    }

    #[tokio::test]
    async fn test_token_list() {
        println!("Listar Token -> ");
        match Token::all(&util::create_client(), &request_token_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar token: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar token");
            }
        }
    }

    #[tokio::test]
    async fn test_token_update() {
        println!("Crear Token -> ");
        let token_id = token::create_token().await;
        println!("Obtener Token por Id {:?} -> ", token_id);
        match Token::patch(
            &util::create_client(),
            &token_id,
            &update_token_request(),
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al actualizar el token: {:?}", rejection);
                panic!("La prueba falló debido a un error al actualizar el token");
            }
        }
    }

    #[tokio::test]
    async fn test_token_yape_create() {
        match Token::yape(&util::create_client(), &create_token_yape_request(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert_eq!(response_json["object"], "token");
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear token de yape: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear token de yape");
            }
        }
    }
}
