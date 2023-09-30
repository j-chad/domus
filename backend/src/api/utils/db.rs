use crate::api::error::{APIError, APIErrorBuilder};
use crate::db::database::{Connection, ConnectionPool};
use tracing::error;

pub(crate) async fn get_db_connection(pool: &ConnectionPool) -> Result<Connection, APIError> {
    let connection: Connection = pool.get().await.map_err(|err| {
        error!(error = %err, "failed to get database connection");
        APIErrorBuilder::from_error(err).build()
    })?;

    Ok(connection)
}
