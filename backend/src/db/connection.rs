use crate::api::shared::errors::APIError;
use actix_web::http::StatusCode;
use actix_web::web;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2;
use r2d2::PooledConnection;
use std::env;

pub(crate) type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_connection(
    pool: &web::Data<DbPool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, APIError> {
    pool.get()
        .map_err(|_e| APIError::from_code(StatusCode::INTERNAL_SERVER_ERROR))
}
