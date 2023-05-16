use axum::{
    routing::{get, post},
    Router,
};

mod routes;
mod schema;
mod services;
mod database;
mod error_handler;
mod models;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let mut app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // make a route with a query parameter
    app = app.route("/game", get(routes::game_routes::get_game_by_day_api));
    app = app.route("/game/current", get(routes::game_routes::get_current_day_api));
    app = app.route("/game/stats", get(routes::game_routes::get_game_day_stats_api));
    
    app = app.route("/game/guest", post(routes::guest_routes::post_game_result_guest));
    app = app.route("/guest/stats", get(routes::guest_routes::get_guest_stats_api));
    app = app.route("/guest/day-stats", get(routes::guest_routes::get_guest_stats_day_api));
    
    app = app.route("/signup", post(routes::user_routes::sign_up_api));
    app = app.route("/login", post(routes::user_routes::login_api));
    app = app.route("/game", post(routes::game_routes::post_game_result_auth));
    app = app.route("/user/stats", get(routes::user_routes::get_user_stats_api));
    app = app.route("/user/day-stats", get(routes::user_routes::get_user_day_stats_api));




    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}