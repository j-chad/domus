use super::error::{APIError, APIErrorBuilder, ErrorType::Unauthorized};
use crate::AppState;
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::AsymmetricPublicKey,
    public,
    token::{TrustedToken, UntrustedToken},
    version4::V4,
    Public,
};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

/// Middleware that validates a PASETO token and adds user info to the request.
pub async fn auth<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, APIError> {
    let untrusted_token = get_token(req.headers())?;
    let trusted_token = validate_token(untrusted_token, &state.settings.auth.public_key)?;

    let current_user = get_user_details(&trusted_token)?;
    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}

fn validate_token(
    token: UntrustedToken<Public, V4>,
    public_key: &str,
) -> Result<TrustedToken, APIError> {
    let mut rules = ClaimsValidationRules::new();
    rules.validate_issuer_with("domus-api.jacksonc.dev");
    rules.validate_audience_with("domus.jacksonc.dev");

    let key = AsymmetricPublicKey::<V4>::try_from(public_key)
        .map_err(|e| APIErrorBuilder::from_error(e).build())?;

    public::verify(&key, &token, &rules, None, None).map_err(|e| {
        APIErrorBuilder::new(Unauthorized)
            .cause(e)
            .detail("The token you provided is not trusted.")
            .build()
    })
}

fn get_token(headers: &HeaderMap<HeaderValue>) -> Result<UntrustedToken<Public, V4>, APIError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
        .ok_or(
            APIErrorBuilder::new(Unauthorized)
                .detail("You are not logged in. Please provide a token.")
                .build(),
        )?;

    UntrustedToken::try_from(token).map_err(|e| {
        APIErrorBuilder::new(Unauthorized)
            .cause(e)
            .detail("The token you provided is invalid.")
            .build()
    })
}

fn get_user_details(token: &TrustedToken) -> Result<CurrentUser, APIError> {
    let claims = token.payload_claims().ok_or(
        APIErrorBuilder::new(Unauthorized)
            .detail("The token you provided is invalid.")
            .build(),
    )?;

    let id = get_user_id(claims)?;
    let email = get_claim(claims, "email")?;
    let first_name = get_claim(claims, "first_name")?;
    let last_name = get_claim(claims, "last_name")?;

    Ok(CurrentUser {
        id,
        email,
        first_name,
        last_name,
    })
}

fn get_user_id(claims: &Claims) -> Result<Uuid, APIError> {
    let sub_claim = get_claim(claims, "sub")?;

    Uuid::parse_str(sub_claim.as_str()).map_err(|e| {
        info!(error = %e, "Token claims contained an invalid subject.");
        APIErrorBuilder::new(Unauthorized)
            .cause(e)
            .detail("The token you provided is invalid.")
            .build()
    })
}

fn get_claim(claims: &Claims, claim_name: &str) -> Result<String, APIError> {
    claims
        .get_claim(claim_name)
        .and_then(|c| c.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            info!(claims = ?claims, claim_name = claim_name, "claim missing from token.");
            APIErrorBuilder::new(Unauthorized)
                .detail("The token you provided is invalid.")
                .build()
        })
}
