use utoipa::Modify;
use utoipa::OpenApi as OpenApiTrait;

use super::auth::api_docs::AuthApiDoc;

struct ApiMerger;

impl Modify for ApiMerger {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.merge(AuthApiDoc::openapi());
    }
}

#[derive(OpenApiTrait)]
#[openapi(
    info(title="Domus API"),
    servers((url="/v1/")),
    modifiers(&ApiMerger),
)]
pub struct ApiDocs {}
