// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        board -> Text,
        day -> Text,
        initial_position -> Text,
    }
}
