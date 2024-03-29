use super::auth::controllers as auth_routes;
use super::auth::models as auth_models;
use super::error;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
	modifiers(&SecurityAddon),
	info(title="Domus API"),
	servers((url="/v1/")),
	paths(
		auth_routes::register,
		auth_routes::login,
		auth_routes::delete_refresh_token,
		auth_routes::refresh_token,
		auth_routes::get_user,
	),
	components(
		schemas(
			error::APIError,
			auth_models::RegisterNewUserRequest,
			auth_models::UserResponse,
			auth_models::LoginUserRequest,
			auth_models::RefreshTokenRequest,
			auth_models::AuthResponse,
		)
	)
)]
pub struct ApiDocs {}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .as_mut()
            .unwrap()
            .security_schemes
            .insert(
                "api_token".to_string(),
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("PASETO")
                        .description(Some("PASETO token"))
                        .build(),
                ),
            );
    }
}

pub fn get_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDocs::openapi())
        .config(
            utoipa_swagger_ui::Config::from("/api-docs/openapi.json")
                .display_request_duration(true)
                .filter(true)
                .request_snippets_enabled(true)
                .persist_authorization(true),
        )
}
