use warp::{http::StatusCode, reject};

#[derive(Debug)]
pub struct CustomRejection {
    pub status: StatusCode,
    pub body: String,
}

impl CustomRejection {
    pub fn from_status_code(status: StatusCode, body: String) -> Self {
        CustomRejection { status, body }
    }
}

impl reject::Reject for CustomRejection {}
