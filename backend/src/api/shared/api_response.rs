// https://github.com/omniti-labs/jsend

use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
enum ResponseStatus {
    Success, // All went well, and (usually) some data was returned.
    Error,   // An error occurred in processing the request, i.e. an exception was thrown
    Fail, // There was a problem with the data submitted, or some pre-condition of the API call wasn't satisfied
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    status: ResponseStatus,
    data: Option<T>,
    message: Option<String>,
    code: Option<u16>,
}

impl<T: Serialize> From<Response<T>> for HttpResponse {
    fn from(value: Response<T>) -> Self {
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

    pub fn fail(data: T) -> Self {
        Self {
            status: ResponseStatus::Fail,
            data: Option::from(data),
            message: None,
            code: None,
        }
    }
}
