// https://github.com/omniti-labs/jsend
use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::{Serialize, Serializer};

#[derive(PartialEq, Debug)]
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
pub struct APIResponse<T: Serialize> {
    status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

pub struct SuccessAPIResponse<T: Serialize> {
    data: T,
}

impl<T: Serialize> Into<APIResponse<T>> for SuccessAPIResponse<T> {
    fn into(self) -> APIResponse<T> {
        APIResponse {
            status: ResponseStatus::Success,
            data: Some(self.data),
            message: None,
        }
    }
}

pub struct ErrorAPIResponse {
    pub(crate) message: String,
}

impl Into<APIResponse<String>> for ErrorAPIResponse {
    fn into(self) -> APIResponse<String> {
        APIResponse {
            status: ResponseStatus::Error,
            data: None,
            message: Some(self.message),
        }
    }
}

pub struct FailAPIResponse<T: Serialize = String> {
    pub(crate) data: Option<T>,
    pub(crate) message: String,
}

impl<T: Serialize> Into<APIResponse<T>> for FailAPIResponse<T> {
    fn into(self) -> APIResponse<T> {
        APIResponse {
            status: ResponseStatus::Fail,
            data: self.data,
            message: Some(self.message),
        }
    }
}

pub trait JSendResponder {
    fn success<T: Serialize>(&mut self, data: T) -> HttpResponse;
    fn error(&mut self, message: String) -> HttpResponse;
    fn fail(&mut self, message: String) -> HttpResponse;
    fn fail_with_data<T: Serialize>(&mut self, message: String, data: T) -> HttpResponse;
}

impl JSendResponder for HttpResponseBuilder {
    fn success<T: Serialize>(&mut self, data: T) -> HttpResponse {
        let response: APIResponse<T> = SuccessAPIResponse { data }.into();
        self.json(response)
    }

    fn error(&mut self, message: String) -> HttpResponse {
        let response: APIResponse<String> = ErrorAPIResponse { message }.into();
        self.json(response)
    }

    fn fail(&mut self, message: String) -> HttpResponse {
        let response: APIResponse<String> = FailAPIResponse {
            data: None,
            message,
        }
        .into();
        self.json(response)
    }

    fn fail_with_data<T: Serialize>(&mut self, message: String, data: T) -> HttpResponse {
        let response: APIResponse<T> = FailAPIResponse {
            data: Some(data),
            message,
        }
        .into();
        self.json(response)
    }
}

// impl APIResponse {
//     pub fn success(code: StatusCode) -> APIResponseBuilder {
//         APIResponseBuilder::new(code, ResponseStatus::Success)
//     }
//
//     pub fn error(code: StatusCode) -> APIResponseBuilder {
//         APIResponseBuilder::new(code, ResponseStatus::Error)
//     }
//
//     pub fn fail(code: StatusCode) -> APIResponseBuilder {
//         APIResponseBuilder::new(code, ResponseStatus::Fail)
//     }
//
//     pub fn new<T: Serialize>(
//         code: StatusCode,
//         status: ResponseStatus,
//         data: Option<T>,
//         message: Option<String>,
//     ) -> Self {
//         Self {
//             status,
//             data,
//             message,
//             code: code.as_u16(),
//         }
//     }
// }
//
// pub struct APIResponseBuilder<T: Serialize = String> {
//     status: ResponseStatus,
//     data: Option<T>,
//     message: Option<String>,
//     code: u16,
// }
//
// impl<T: Serialize> APIResponseBuilder<T> {
//     pub fn new(code: StatusCode, status: ResponseStatus) -> Self {
//         Self {
//             status,
//             data: None,
//             message: None,
//             code: code.as_u16(),
//         }
//     }
//
//     pub fn data(mut self, data: T) -> Self {
//         self.data = Some(data);
//         self
//     }
//
//     pub fn message(mut self, message: String) -> Self {
//         if self.status == ResponseStatus::Success {
//             panic!("Cannot set message on success response");
//         }
//
//         self.message = Some(message);
//         self
//     }
//
//     pub fn build(self) -> APIResponse<T> {
//         APIResponse {
//             status: self.status,
//             data: self.data,
//             message: self.message,
//             code: Some(self.code),
//         }
//     }
// }
