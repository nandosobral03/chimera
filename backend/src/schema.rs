// @generated automatically by Diesel CLI.

diesel::table! {
    day_stats (day) {
        day -> Text,
        total_games -> Integer,
        total_wins -> Integer,
        aggregated_board_stats -> Text,
    }
}

diesel::table! {
    games (id) {
        id -> Integer,
        board -> Text,
        day -> Text,
        initial_position -> Text,
    }
}

diesel::table! {
    guest_day_stats (guest_id, day) {
        guest_id -> Nullable<Text>,
        day -> Text,
        status -> Text,
        board -> Text,
    }
}

diesel::table! {
    guests (id) {
        id -> Text,
        games_played -> Integer,
        wins -> Integer,
    }
}

diesel::table! {
    user_day_stats (user_id, day) {
        user_id -> Integer,
        day -> Text,
        status -> Text,
        board -> Text,
        last_move -> Nullable<Text>,
    }
}

diesel::table! {
    user_stats (user_id) {
        user_id -> Integer,
        total_games -> Integer,
        total_wins -> Integer,
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

diesel::joinable!(user_day_stats -> users (user_id));
diesel::joinable!(user_stats -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    day_stats,
    games,
    guest_day_stats,
    guests,
    user_day_stats,
    user_stats,
    users,
);
