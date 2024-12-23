use std::{collections::HashMap, fmt};

#[derive(Debug,)] // Implementing Debug for CustomException
pub struct CustomException {
    error_data: HashMap<String, String,>,
}

// Implementing Error trait for CustomException
impl warp::reject::Reject for CustomException {}

// Implement Display for CustomException to provide a string representation
impl fmt::Display for CustomException {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        // Serialize the error_data HashMap into a JSON string
        match serde_json::to_string(&self.error_data,) {
            Ok(json_str,) => write!(f, "{}", json_str), // If serialization is successful, write
            // the JSON string
            Err(_,) => write!(f, "{{\"error\": \"Failed to serialize error data\"}}"), /* Handle serialization errors */
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
        // Convert serde_json::Error into CustomException
        // Here you might format the error message or include additional error
        // information
        CustomException::new(&format!("JSON error: {}", err),)
    }
}
