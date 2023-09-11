use crate::db::schema::users;
use diesel::dsl::{AsSelect, Eq, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

type All = Select<users::table, AsSelect<User, Pg>>;
type WithEmail<'a> = Eq<users::email, &'a str>;

impl User {
    pub fn all() -> All {
        users::table.select(User::as_select())
    }

    pub fn by_email(email: &str) -> WithEmail {
        users::email.eq(email)
    }
}
