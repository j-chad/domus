use super::user::User;
use crate::db::schema::refresh_tokens;
use diesel::prelude::*;
use uuid;

#[derive(Queryable, Associations, Identifiable, Selectable)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct NewRefreshToken<'a> {
    pub user_id: &'a uuid::Uuid,
}

pub fn create_refresh_token(
    conn: &mut PgConnection,
    user_id: &uuid::Uuid,
) -> QueryResult<RefreshToken> {
    diesel::insert_into(refresh_tokens::table)
        .values(NewRefreshToken { user_id })
        .get_result(conn)
}
