use deadpool::managed::Object;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::env;

pub type Connection = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type ConnectionPool =
    deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>, Connection>;

pub fn get_connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .max_size(16)
        .build()
        .expect("Failed to create database pool.")
}
