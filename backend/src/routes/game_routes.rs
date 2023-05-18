use axum::{response::IntoResponse, Json, extract::{ Query}};
use axum::http::HeaderMap;
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;
use crate::{models::{game::GameResult}, services::user_service::{save_won_game, save_lost_game}};

use super::utils::{decode_token, handle_error};

#[derive(Deserialize)]
pub struct DayQuery {
    pub day:String,
}

pub async fn get_game_by_day_api(Query (day_query): Query<DayQuery>) -> impl IntoResponse {
    let day = day_query.day;
    let game = crate::services::game_service::get_game_by_day(&day);
    match game {
        Ok(game) => Json(json!(game)).into_response(),
        Err(err) => handle_error(err).into_response()
    }
}


pub async fn get_current_day_api() -> impl IntoResponse {
    let day = chrono::Local::now().format("%d/%m/%Y").to_string();
    let game = crate::services::game_service::get_game_by_day(&day);
    match game {
        Ok(game) => Json(json!(game)).into_response(),
        Err(err) => handle_error(err).into_response()
    }
}

pub async fn post_game_result_auth(
    headers: HeaderMap,
    Json(result): Json<GameResult>,
) -> impl IntoResponse {
    let current_day = chrono::Local::now().format("%d/%m/%Y").to_string();
    match decode_token(headers){
        Ok(token) => {
            match result.exploded{
                Some(_) => {
                    match save_lost_game(result, token.user, current_day).await{
                        Ok(_) => {
                            Json(json!({"message": "Game saved"})).into_response()
                        },
                        Err(err) => handle_error(err).into_response()
                    }
                },
                None => {
                    match save_won_game(result, token.user, current_day).await{
                        Ok(_) => {
                            Json(json!({"message": "Game saved"})).into_response()
                        },
                        Err(err) => handle_error(err).into_response()
                    }
                }
            }
        },
        Err(err) => handle_error(err).into_response()
    }
}




pub async fn get_game_day_stats_api (
    Query (day_query): Query<DayQuery>
) -> impl IntoResponse {
    let day = day_query.day;
    let stats = crate::services::game_service::get_day_stats(&day);
    match stats {
        Ok(stats) => Json(json!(stats)).into_response(),
        Err(err) => handle_error(err).into_response()
    }
}

pub async fn get_time_until_next_game() -> impl IntoResponse {
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let now_naive = now.naive_local().date();
    let now_naive_datetime = now.naive_local();
    if let Some(tomorrow) = now_naive.succ_opt(){
        if let Some(tomorrow_midnight) = tomorrow.and_hms_opt(0, 0, 0){
            let time_until_next_game = tomorrow_midnight.signed_duration_since(now_naive_datetime);
            return Json(json!({"time_until_next_game": time_until_next_game.num_seconds()})).into_response();
        }
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": "Could not calculate time until next game"})),
    ).into_response()

}