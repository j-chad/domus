use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use const_format::concatcp;
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

const ERROR_URI: &str = "tag:domus@jacksonc.dev,2023:errors/";

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("An unknown error has occurred.")]
    Unknown,

    #[error("An unknown error has occurred.")]
    ForeignError(#[from] anyhow::Error),

    #[error("Your request is not valid.")]
    ValidationError,

    #[error("A user with that email already exists.")]
    UserAlreadyExists,

    #[error("Login Incorrect.")]
    LoginIncorrect,

    #[error("You have not been authorized to perform this action.")]
    Unauthorized,

    #[error("You are not allowed to perform this action.")]
    Forbidden,
}

impl ErrorType {
    pub fn get_type(&self) -> &'static str {
        match self {
            ErrorType::Unknown | ErrorType::ForeignError(_) => "about:blank",
            ErrorType::ValidationError => concatcp!(ERROR_URI, "validation-error"),
            ErrorType::UserAlreadyExists => concatcp!(ERROR_URI, "user-already-exists"),
            ErrorType::LoginIncorrect => concatcp!(ERROR_URI, "login-incorrect"),
            ErrorType::Unauthorized => concatcp!(ERROR_URI, "unauthorized"),
            ErrorType::Forbidden => concatcp!(ERROR_URI, "forbidden"),
        }
    }

    pub fn get_title(&self) -> String {
        self.to_string()
    }

    pub fn get_status(&self) -> StatusCode {
        match self {
            ErrorType::Unknown | ErrorType::ForeignError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::ValidationError => StatusCode::BAD_REQUEST,
            ErrorType::UserAlreadyExists => StatusCode::CONFLICT,
            ErrorType::LoginIncorrect => StatusCode::UNAUTHORIZED,
            ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorType::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    pub fn get_detail(&self) -> Option<&'static str> {
        match self {
            ErrorType::LoginIncorrect => Some("The email or password you entered is incorrect. Please check your credentials and try again."),
            _ => None,
        }
    }
}

/// RFC 9457 Compliant Error Response
#[derive(Serialize, Debug, ToSchema)]
pub struct APIError {
    /// A URI reference that identifies the problem type. This is the primary identifier for the problem type.
    ///
    /// When this member is not present, its value is assumed to be "about:blank"
    #[schema(example = "https://example.com/problems/fire")]
    #[serde(rename = "type")]
    error_type: String,

    /// The HTTP status code generated by the origin server for this occurrence of the problem.
    #[schema(example = "500", value_type = u16, format = "HTTPStatusCode")]
    #[serde(serialize_with = "serialize_status_code")]
    status: StatusCode,

    /// A short, human-readable summary of the problem type.
    #[schema(example = "The world is on fire.")]
    title: String,

    /// A human-readable explanation specific to this occurrence of the problem.
    #[schema(
        example = "A fire has started in the server room and the fire suppression system has failed. Please evacuate the building immediately."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,

    /// A URI reference that identifies the specific occurrence of the problem.
    #[schema(example = "https://example.com/events/fire/42")]
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,

    #[schema(additional_properties = true)]
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Box<Option<HashMap<String, Value>>>,
}

impl From<ErrorType> for APIError {
    fn from(value: ErrorType) -> Self {
        if let ErrorType::ForeignError(e) = &value {
            return APIErrorBuilder::from_error(e).build();
        }

        APIErrorBuilder::new(value).build()
    }
}

impl From<anyhow::Error> for APIError {
    fn from(error: anyhow::Error) -> Self {
        APIError::from(ErrorType::ForeignError(error))
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let resp = Response::builder()
            .status(self.status)
            .header("Content-Type", "application/problem+json")
            .body(Json(self).into_response().into_body());

        // This should never fail, but if it does, we want to know about it.
        resp.unwrap_or_else(|e| {
            error!(error = %e, "Failed to build error response");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build error response",
            )
                .into_response()
        })
    }
}

/// Builds an RFC 9457 compliant error response.
pub struct APIErrorBuilder {
    error_type: ErrorType,
    detail: Option<String>,
    instance: Option<String>,
    extra: Option<HashMap<String, Value>>,
}

impl APIErrorBuilder {
    /// Create a new APIErrorBuilder with the given error type.
    ///
    /// This is the recommended way to create an APIErrorBuilder.
    pub fn new(error: ErrorType) -> Self {
        Self {
            error_type: error,
            detail: None,
            instance: None,
            extra: None,
        }
    }

    /// Create a new unknown error from the given error.
    ///
    /// This is a shorthand for `APIErrorBuilder::new(Unknown).cause(error)`.
    pub fn from_error(error: impl Display) -> Self {
        Self::new(ErrorType::Unknown).cause(error)
    }

    /// Adds a cause field to the error.
    ///
    /// Shorthand for `with_field("cause", error.to_string().into())`.
    pub fn cause(self, error: impl Display) -> Self {
        self.with_field("cause", format!("{}", error).into())
    }

    /// Add additional information to the error.
    ///
    /// RFC 9457:
    /// > The "detail" member is a JSON string containing a human-readable explanation specific to this occurrence of the problem.
    /// >
    /// > The "detail" string, if present, ought to focus on helping the client correct the problem,
    /// > rather than giving debugging information.
    pub fn detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.to_string());
        self
    }

    /// Add an instance URI to the error.
    ///
    /// RFC 9457:
    /// > The "instance" member is a JSON string containing a URI reference that identifies the specific occurrence of the problem.
    /// >
    /// > When the "instance" URI is dereferenceable, the problem details object can be fetched from it.
    /// >
    /// > When the "instance" URI is not dereferenceable, it serves as a unique identifier for the problem occurrence that may be of significance to the server but is opaque to the client.
    #[allow(dead_code)] // TODO: remove after first use
    pub fn instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }

    /// Add an additional field to the error.
    ///
    /// RFC 9457:
    /// > Problem type definitions MAY extend the problem details object with additional members that are specific to that problem type.
    pub fn with_field(mut self, field: &str, value: Value) -> Self {
        self.extra
            .get_or_insert_with(HashMap::new)
            .insert(field.to_string(), value);
        self
    }

    pub fn build(self) -> APIError {
        APIError {
            status: self.error_type.get_status(),
            title: self.error_type.get_title().to_string(),
            detail: self
                .detail
                .or(self.error_type.get_detail().map(|s| s.to_string())),
            instance: self.instance,
            extra: Box::from(self.extra),
            error_type: self.error_type.get_type().to_string(),
        }
    }
}

impl Default for APIErrorBuilder {
    fn default() -> Self {
        Self::new(ErrorType::Unknown)
    }
}

fn serialize_status_code<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(value.as_u16())
}
