use hyper::StatusCode;

pub fn get_status(status_code: u16) -> StatusCode {
    return warp::http::StatusCode::from_u16(status_code)
    .unwrap_or(warp::http::StatusCode::INTERNAL_SERVER_ERROR);
}

