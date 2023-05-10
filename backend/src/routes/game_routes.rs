use axum::{response::IntoResponse, Json, extract::{ Query}};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;


#[derive(Deserialize)]
pub struct DayQuery {
    day:String,
}

pub async fn get_game_by_day_api(Query (day_query): Query<DayQuery>) -> impl IntoResponse {
    let day = day_query.day;
    let game = crate::services::game_service::get_game_by_day(&day);
    match game {
        Ok(game) => Json(json!(game)).into_response(),
        Err(err) =>  {
            match err.code {
                400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
                404 => (StatusCode::NOT_FOUND, Json(json!({"error": err.message}))).into_response(),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
            }
        }
    }
}
