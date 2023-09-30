use crate::config::Settings;
use deadpool::managed::Object;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type Connection = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type ConnectionPool =
    deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>, Connection>;

pub fn get_connection_pool(settings: &Settings) -> ConnectionPool {
    let database_url = &settings.database.url;
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .max_size(settings.database.max_pool_size as usize)
        .build()
        .expect("Failed to create database pool.")
}
