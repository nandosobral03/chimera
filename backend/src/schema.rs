// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        board -> Text,
        day -> Text,
        initial_position -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password_hash -> Text,
        salt -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    games,
    users,
);
