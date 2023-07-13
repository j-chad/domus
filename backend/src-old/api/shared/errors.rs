use super::api_response::APIResponse;
use crate::api::shared::api_response::{ErrorAPIResponse, FailAPIResponse};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::r2d2::PoolError;
use log::warn;
use std::error::Error;
use std::fmt;
use validator::ValidationErrors;

#[derive(PartialEq, Debug)]
pub enum APIErrorType {
    UnknownServerError,
    JsonClientError,
    DbError,
    ValidationFailure,
    Unauthorised,
}

impl fmt::Display for APIErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIErrorType::DbError => write!(f, "database error"),
            APIErrorType::ValidationFailure => write!(f, "validation failure"),
            _ => write!(f, "unknown server error"),
        }
    }
}

impl ResponseError for APIErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

#[derive(Debug)]
pub struct APIError {
    pub code: StatusCode,
    pub message: Option<String>,
}

impl From<PoolError> for APIError {
    fn from(_error: PoolError) -> APIError {
        APIError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(_error.to_string()),
        }
    }
}

impl From<ValidationErrors> for APIError {
    fn from(value: ValidationErrors) -> Self {
        APIError {
            code: StatusCode::BAD_REQUEST,
            message: Some(value.to_string()),
        }
    }
}

impl From<Box<dyn Error>> for APIError {
    fn from(_error: Box<dyn Error>) -> APIError {
        warn!("Unknown Server Error: {}", _error);
        APIError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(_error.to_string()),
        }
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl APIError {
    fn message(&self) -> String {
        if let Some(message) = &self.message {
            message.to_string()
        } else {
            self.code.to_string()
        }
    }

    pub fn from_code(code: StatusCode) -> APIError {
        APIError {
            code,
            message: None,
        }
    }
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        let response: APIResponse<String>;
        if self.status_code().is_client_error() {
            response = FailAPIResponse {
                message: self.message(),
                data: None,
            }
            .into();
        } else {
            response = ErrorAPIResponse {
                message: self.message(),
            }
            .into();
        }
        HttpResponse::build(self.status_code()).json(response)
    }
}
