use utoipa::Modify;
use utoipa::OpenApi as OpenApiTrait;

use super::{controller, model};

struct AuthScopeAddon;

impl Modify for AuthScopeAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let keys: Vec<String> = openapi.paths.paths.keys().cloned().collect();

        for key in keys {
            let new_key = format!("/auth{}", key);
            if let Some(value) = openapi.paths.paths.remove(&key) {
                openapi.paths.paths.insert(new_key, value);
            }
        }
    }
}

#[derive(OpenApiTrait)]
#[openapi(
    paths(controller::register, controller::login),
    components(schemas(model::RegisterNewUser, model::LoginUser, model::AuthResponse)),
    modifiers(&AuthScopeAddon),
    tags((name="Auth", description="Endpoints for authentication and user management"))
)]
pub struct AuthApiDoc;
