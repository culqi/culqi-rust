use tokio;
use LibCulqi::*;
mod config;
use config::credentials::{SECRET_KEY};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[tokio::test]
    async fn test_order_delete() {
        match delete("tokens", "ord_test_20HjpSkdDlSdoHEC", SECRET_KEY).await {
            Ok((response_text, _status_code)) => {
                println!("Respuesta del servidor: {}", response_text)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
