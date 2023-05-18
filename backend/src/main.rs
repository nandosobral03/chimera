use axum::{
    routing::{get, post},
    Router,
};

use tower_http::cors::{CorsLayer};

mod routes;
mod schema;
mod services;
mod database;
mod error_handler;
mod models;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/game", get(routes::game_routes::get_game_by_day_api))
        .route("/game/current", get(routes::game_routes::get_current_day_api))
        .route("/game/stats", get(routes::game_routes::get_game_day_stats_api))
        .route("/game/guest", post(routes::guest_routes::post_game_result_guest))
        .route("/guest/stats", get(routes::guest_routes::get_guest_stats_api))
        .route("/guest/day-stats", get(routes::guest_routes::get_guest_stats_day_api))
        .route("/guest/current", get(routes::guest_routes::get_guest_current_stats_day_api))
        .route("/signup", post(routes::user_routes::sign_up_api))
        .route("/login", post(routes::user_routes::login_api))
        .route("/game", post(routes::game_routes::post_game_result_auth))
        .route("/user/stats", get(routes::user_routes::get_user_stats_api))
        .route("/user/day-stats", get(routes::user_routes::get_user_day_stats_api))
        .route("/user/current", get(routes::user_routes::get_user_current_day_stats_api))
        .route("/game/next", get(routes::game_routes::get_time_until_next_game))
        .layer(CorsLayer::permissive());

    // run it with hyper on localhost:3000
    println!("Listening on port 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}