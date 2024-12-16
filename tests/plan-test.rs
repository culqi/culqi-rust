use tokio;
use LibCulqi::*;
mod config;
use config::credentials::{PUBLIC_KEY, SECRET_KEY};
mod request {
    pub mod plan;
}
use request::plan::{REQUEST_PLAN_ALL, REQUEST_PLAN_CREATE, REQUEST_PLAN_UPDATE};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_plan_create() {
        // Ejemplo de cómo usar la función

        match create(REQUEST_PLAN_CREATE, "plans", PUBLIC_KEY, SECRET_KEY).await {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_plan_update() {
        // Ejemplo de cómo usar la función

        match update(
            REQUEST_PLAN_UPDATE,
            "plans",
            "pln_live_oUr3os1vYacQ4wI8",
            PUBLIC_KEY,
            SECRET_KEY,
        )
        .await
        {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 200, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_plan_get() {
        match get("plans", "pln_live_oUr3os1vYacQ4wI8", SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_all() {
        match all("plans", REQUEST_PLAN_ALL, SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_plan_delete() {
        match delete("plans", "pln_live_oUr3os1vYacQ8wI9", SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
