use crate::api::shared::api_response::Response;
use actix_web::error::{InternalError, JsonPayloadError};
use log::error;
use serde_json::error::Category;
use serde_json::Error as JsonError;

pub fn handle_json_error(err: JsonPayloadError) -> InternalError<JsonPayloadError> {
    let response = get_response(&err);
    InternalError::from_response(err, response.into())
}

fn get_response(err: &JsonPayloadError) -> Response<String> {
    match err {
        JsonPayloadError::OverflowKnownLength { .. }
        | JsonPayloadError::Overflow { .. }
        | JsonPayloadError::ContentType => Response::fail(err.to_string(), None),
        JsonPayloadError::Deserialize(e) => get_json_error_message(e),
        JsonPayloadError::Serialize(_) => {
            Response::error("failed to serialize response".to_string(), None, None)
        }
        JsonPayloadError::Payload(e) => Response::fail(e.to_string(), None),
        _ => {
            error!("Unhandled JsonPayloadError: {:?}", err);
            Response::error("an unknown error occurred".to_string(), None, None)
        }
    }
}

fn get_json_error_message(err: &JsonError) -> Response<String> {
    match err.classify() {
        Category::Io => Response::error(err.to_string(), None, None),
        Category::Syntax | Category::Eof | Category::Data => Response::fail(err.to_string(), None),
    }
}
