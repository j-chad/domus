use utoipa::OpenApi;

use crate::handlers::auth;
use crate::models::auth::{LoginUserRequest, RefreshTokenRequest, RegisterNewUserRequest};

#[derive(OpenApi)]
#[openapi(
	info(title="Domus API"),
	servers((url="/v1/")),
	paths(
		auth::register,
		auth::login,
		auth::logout,
		auth::refresh_token,
		auth::get_user,
	),
	components(
		schemas(
			RegisterNewUserRequest,
			LoginUserRequest,
			RefreshTokenRequest,
		)
	)
)]
pub struct ApiDocs {}
