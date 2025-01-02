use std::{collections::HashMap, fmt};
#[derive(Debug,)]
pub struct CustomException {
    error_data: HashMap<String, String,>,
}

impl warp::reject::Reject for CustomException {}

impl fmt::Display for CustomException {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match serde_json::to_string(&self.error_data,) {
            Ok(json_str,) => write!(f, "{}", json_str),
            Err(_,) => write!(f, "{{\"error\": \"Failed to serialize error data\"}}"),
        }
    }
}

// Implement CustomException methods
impl CustomException {
    pub fn new(merchant_message: &str,) -> CustomException {
        let mut error_data = HashMap::new();
        error_data.insert("object".to_string(), "error".to_string(),);
        error_data.insert("type".to_string(), "param_error".to_string(),);
        error_data.insert("merchant_message".to_string(), merchant_message.to_string(),);
        error_data.insert("user_message".to_string(), merchant_message.to_string(),);

        CustomException { error_data, }
    }
}

impl From<serde_json::Error,> for CustomException {
    fn from(err: serde_json::Error,) -> Self {
        CustomException::new(&format!("JSON error: {}", err),)
    }
}
