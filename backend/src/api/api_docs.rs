use utoipa::OpenApi;

use super::auth::controllers as auth_routes;
use super::auth::models as auth_models;

#[derive(OpenApi)]
#[openapi(
	info(title="Domus API"),
	servers((url="/v1/")),
	paths(
		auth_routes::register,
		auth_routes::login,
		auth_routes::logout,
		auth_routes::refresh_token,
		auth_routes::get_user,
	),
	components(
		schemas(
			auth_models::RegisterNewUserRequest,
			auth_models::LoginUserRequest,
			auth_models::RefreshTokenRequest,
		)
	)
)]
pub struct ApiDocs {}
