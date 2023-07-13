use crate::api::shared::errors::APIError;
use actix_web::error::JsonPayloadError;
use log::error;
pub fn handle_json_error(err: JsonPayloadError) -> APIError {
    get_response(err)
}

fn get_response(err: JsonPayloadError) -> APIError {
    match err {
        JsonPayloadError::OverflowKnownLength { .. }
        | JsonPayloadError::Overflow { .. }
        | JsonPayloadError::ContentType => APIError {
            code: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(err.to_string()),
        },
        JsonPayloadError::Deserialize(e) => APIError {
            code: actix_web::http::StatusCode::BAD_REQUEST,
            message: Some(e.to_string()),
        },
        JsonPayloadError::Serialize(_) => APIError {
            code: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: Some("failed to serialize response".to_string()),
        },
        JsonPayloadError::Payload(e) => APIError {
            code: actix_web::http::StatusCode::BAD_REQUEST,
            message: Some(e.to_string()),
        },
        _ => {
            error!("Unhandled JsonPayloadError: {:?}", err);
            APIError {
                code: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                message: None,
            }
        }
    }
}
