use tokio;
use LibCulqi::*;
mod config; // Esto importa el archivo config.rs en la carpeta tests
use config::credentials::{PUBLIC_KEY, SECRET_KEY};
mod request { 
    pub mod subscription;

}
use request::subscription::{REQUEST_SUBSCRIṔTTION_CREATE, REQUEST_SUBSCRIṔTTION_UPDATE, REQUEST_SUBSCRIṔTTION_ALL};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[tokio::test]
    async fn test_subscription_create() {
        match create(
            REQUEST_SUBSCRIṔTTION_CREATE,
            "subscriptions",
            PUBLIC_KEY,
            SECRET_KEY,
        )
        .await
        {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
                assert_eq!(status_code, 201, "Expected status code 201");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_subscription_update() {
        match update(
            REQUEST_SUBSCRIṔTTION_UPDATE,
            "subscriptions",
            "sxn_live_neFrhLr8QvozBdWn",
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
    async fn test_subscription_all() {
        match all("subscriptions", REQUEST_SUBSCRIṔTTION_ALL, SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_subscription_get() {
        match get("subscriptions", "sxn_live_neFrhLrX8vozBdWn", SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_subscription_delete() {
        match delete("subscriptions", "sxn_live_neFrh8rXQvozBdWn", SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

}
