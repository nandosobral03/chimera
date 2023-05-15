use axum::{response::IntoResponse, Json, extract::{ Query}};
use axum::http::HeaderMap;

use hyper::{StatusCode};
use serde::Deserialize;
use serde_json::json;
use crate::{models::{game::GameResult}, services::user_service::{save_won_game, save_lost_game}};

use super::utils::decode_token;

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


pub async fn get_curreny_day_api() -> impl IntoResponse {
    let day = chrono::Local::now().format("%d/%m/%Y").to_string();
    let game = crate::services::game_service::get_game_by_day(&day);
    match game {
        Ok(game) => Json(json!(game)).into_response(),
        Err(err) =>  {
            match err.code {
                400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
            }
        }
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
                        Err(err) => {
                            match err.code {
                                400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
                                409 => (StatusCode::CONFLICT, Json(json!({"error": err.message}))).into_response(),
                                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
                            }
                        }
                    }
                },
                None => {
                    match save_won_game(result, token.user, current_day).await{
                        Ok(_) => {
                            Json(json!({"message": "Game saved"})).into_response()
                        },
                        Err(err) => {
                            match err.code {
                                400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
                                409 => (StatusCode::CONFLICT, Json(json!({"error": err.message}))).into_response(),
                                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
                            }
                        }
                    }
                }
            }
        },
        Err(err) =>  match err.code {
            400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
            401 => (StatusCode::UNAUTHORIZED, Json(json!({"error": err.message}))).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
        }
    }
}


