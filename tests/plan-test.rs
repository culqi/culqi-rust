use LibCulqi::culqi::plan::Plan;
mod config;
mod header;
mod request;
mod utils;

#[cfg(test)]
mod plan_tests {
    use header::header_rsa;
    use request::plan::{create_plan_request, request_plan_all, request_plan_update};
    use serial_test::serial;
    use utils::{plan, util};

    use super::*;
    #[tokio::test]
    #[serial]
    async fn test_plan_create() {
        println!("Crear Plan -> ");
        match Plan::create(&util::create_client(), &create_plan_request(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["slug"].is_string(),
                    "El campo 'slug' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el plan: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el plan");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_plan_create_encrypt() {
        println!("Crear Plan con llaves RSA-> ");
        match Plan::create(
            &util::create_client_encrypt(),
            &create_plan_request(),
            Some(header_rsa::get_header_encrypt(),),
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 201,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["slug"].is_string(),
                    "El campo 'slug' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al crear el plan: {:?}", rejection);
                panic!("La prueba falló debido a un error al crear el plan");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_plan_update() {
        println!("Crear Plan -> ");
        let plan_id = plan::create_plan().await;
        println!("Actualizar Plan -> ");
        match Plan::patch(
            &util::create_client(),
            &plan_id,
            &request_plan_update(),
            None,
        )
        .await
        {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["slug"].is_string(),
                    "El campo 'slug' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al eliminar el plan: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el plan");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_plan_get() {
        println!("Crear Plan -> ");
        let plan_id = plan::create_plan().await;
        println!("Obtener Plan por Id {:?} -> ", plan_id);
        match Plan::get(&util::create_client(), &plan_id, None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json = util::parse_response_body(response,).await;
                assert!(
                    response_json["slug"].is_string(),
                    "El campo 'slug' no es una cadena"
                );
                assert!(
                    response_json["id"].is_string(),
                    "El campo 'id' no es una cadena"
                );
            }
            Err(rejection,) => {
                println!("Error al obtener plan por id: {:?}", rejection);
                panic!("La prueba falló debido a un error al obtener plan por id");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_plan_all() {
        println!("Listar Plan -> ");
        match Plan::all(&util::create_client(), &request_plan_all(), None,).await {
            Ok(response,) => {
                util::assert_status(&response, 200,);
                let response_json: serde_json::Value = util::parse_response_body(response,).await;
                assert!(
                    response_json["paging"].is_object(),
                    "El campo 'paging' debe ser type object"
                );
            }
            Err(rejection,) => {
                println!("Error al listar plan: {:?}", rejection);
                panic!("La prueba falló debido a un error al listar plan");
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_plan_delete() {
        println!("Listar Plan -> ");
        let plan_id = plan::create_plan().await;
        println!("Eliminar Plan por Id {:?} -> ", plan_id);
        match Plan::delete(&util::create_client(), &plan_id, None,).await {
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
                println!("Error al eliminar el plan: {:?}", rejection);
                panic!("La prueba falló debido a un error al eliminar el plan");
            }
        }
    }
}
