// https://github.com/omniti-labs/jsend

use actix_web::HttpResponse;
use log::error;
use serde::{Serialize, Serializer};

enum ResponseStatus {
    Success, // All went well, and (usually) some data was returned.
    Error,   // An error occurred in processing the request, i.e. an exception was thrown
    Fail, // There was a problem with the data submitted, or some pre-condition of the API call wasn't satisfied
}

impl Serialize for ResponseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ResponseStatus::Success => serializer.serialize_str("success"),
            ResponseStatus::Error => serializer.serialize_str("error"),
            ResponseStatus::Fail => serializer.serialize_str("fail"),
        }
    }
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<u16>,
}

impl<T: Serialize> From<Response<T>> for HttpResponse {
    fn from(value: Response<T>) -> Self {
        if let Some(code) = value.code {
            let status_code = actix_web::http::StatusCode::from_u16(code);
            return if let Ok(status_code) = status_code {
                HttpResponse::build(status_code).json(value)
            } else {
                error!("Invalid status code: {}", code);
                HttpResponse::InternalServerError().finish()
            };
        }

        match value.status {
            ResponseStatus::Success => HttpResponse::Ok().json(value),
            ResponseStatus::Error => HttpResponse::InternalServerError().json(value),
            ResponseStatus::Fail => HttpResponse::BadRequest().json(value),
        }
    }
}

impl<T: Serialize> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: ResponseStatus::Success,
            data: Option::from(data),
            message: None,
            code: None,
        }
    }

    pub fn error(message: String, code: Option<u16>, data: Option<T>) -> Self {
        Self {
            status: ResponseStatus::Error,
            data,
            message: Option::from(message),
            code,
        }
    }

    pub fn fail(message: String, data: Option<T>) -> Self {
        Self {
            status: ResponseStatus::Fail,
            data,
            message: Option::from(message),
            code: None,
        }
    }
}
