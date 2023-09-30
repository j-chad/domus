use super::error::APIError;
use crate::api::error::APIErrorBuilder;
use crate::api::error::ErrorType::{ForeignError, Unauthorized};
use crate::config::Auth;
use crate::AppState;
use axum::extract::State;
use axum::http::{header, Request};
use axum::middleware::Next;
use axum::response::IntoResponse;
use pasetors::claims::ClaimsValidationRules;
use pasetors::keys::AsymmetricPublicKey;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{public, Public};

async fn auth<B>(
    State(state): &State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, APIError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or(
            APIErrorBuilder::error(Unauthorized)
                .detail("You are not logged in. Please provide a token.")
                .build(),
        )?;

    let untrusted_token = UntrustedToken::<Public, V4>::try_from(&token).or(Err(
        APIErrorBuilder::error(Unauthorized)
            .detail("The token you provided is invalid.")
            .build(),
    ))?;

    let mut rules = ClaimsValidationRules::new();
    rules.validate_issuer_with("domus-api.jacksonc.dev");
    rules.validate_audience_with("domus.jacksonc.dev");

    let Auth { public_key, .. } = &state.settings.auth;
    let key = AsymmetricPublicKey::<V4>::try_from(public_key.as_str()).map_err(|_| {
        APIErrorBuilder::error(Unauthorized)
            .detail("The token you provided is invalid.")
            .build()
    })?;
    let trusted_token = public::verify(&key, &untrusted_token, &rules, None, None).or(Err(
        APIErrorBuilder::error(Unauthorized)
            .detail("The token you provided is not trusted.")
            .build(),
    ))?;

    Ok(next.run(req).await)
}
