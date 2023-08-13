// @generated automatically by Diesel CLI.

diesel::table! {
    refresh_tokens (id) {
        id -> Text,
        user_id -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    refresh_tokens,
    users,
);
