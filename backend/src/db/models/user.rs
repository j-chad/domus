use crate::db::schema;
use diesel::prelude::*;
use uuid;

#[derive(Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
}

pub fn create_user(conn: &mut PgConnection, user: NewUser) -> QueryResult<User> {
    diesel::insert_into(schema::users::table)
        .values(&user)
        .get_result(conn)
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> QueryResult<User> {
    schema::users::table
        .filter(schema::users::email.eq(email))
        .first(conn)
}
