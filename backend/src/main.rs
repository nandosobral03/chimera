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
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // make a route with a query parameter
    let app = app.route("/game", get(routes::game_routes::get_game_by_day_api));

    let app = app.route("/signup", post(routes::user_routes::sign_up_api));
    let app = app.route("/login", post(routes::user_routes::login_api));
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}