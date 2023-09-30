use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl RefreshToken {
    pub(crate) fn is_expired(&self) -> bool {
        self.expires_at < chrono::Utc::now().naive_utc()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::refresh_tokens)]
pub struct NewRefreshToken {
    pub user_id: Uuid,
}
