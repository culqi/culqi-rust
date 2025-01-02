use warp::{Reply, http::StatusCode, reply::Response};

pub fn create_warp_response(body: String, status_code: u16,) -> Response {
    let status = StatusCode::from_u16(status_code,).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR,);
    warp::reply::with_status(warp::reply::html(body,), status,).into_response()
}

pub async fn handle_response(
    result: Result<(String, u16,), (String, u16,),>,
) -> Result<warp::reply::Response, warp::Rejection,> {
    match result {
        Ok((response_text, status_code,),) => {
            Ok(create_warp_response(response_text, status_code,),)
        }
        Err((error_message, status_code,),) => {
            Ok(create_warp_response(error_message, status_code,),)
        }
    }
}
