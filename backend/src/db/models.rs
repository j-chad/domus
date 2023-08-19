use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
