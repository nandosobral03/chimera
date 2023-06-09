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
        guest_id -> Text,
        day -> Text,
        status -> Text,
        board -> Text,
        last_move -> Nullable<Text>,
        flags -> Text,
        time_taken -> Integer,
    }
}

diesel::table! {
    guests (id) {
        id -> Text,
        total_games -> Integer,
        total_wins -> Integer,
        win_streak -> Integer,
    }
}

diesel::table! {
    user_day_stats (user_id, day) {
        user_id -> Integer,
        day -> Text,
        status -> Text,
        board -> Text,
        last_move -> Nullable<Text>,
        flags -> Text,
        time_taken -> Integer,
    }
}

diesel::table! {
    user_stats (user_id) {
        user_id -> Integer,
        win_streak -> Integer,
        total_games -> Integer,
        total_wins -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        salt -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(guest_day_stats -> guests (guest_id));
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
