use crate::{
    models::game::GameResult,
    services::guest_service::{
        get_guest_day_stats, get_guest_stats, save_lost_game_guest, save_won_game_guest,
    },
};
use axum::http::HeaderMap;
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use super::{game_routes::DayQuery, utils::handle_error};

pub async fn post_game_result_guest(
    headers: HeaderMap,
    Json(result): Json<GameResult>,
) -> impl IntoResponse {
    let current_day = chrono::Local::now().format("%d/%m/%Y").to_string();

    let header = headers.get("guest_id");
    if header.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Guest id not provided"})),
        )
            .into_response();
    }

    match headers
        .get("guest_id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
    {
        safe_guest_id => match result.exploded {
            Some(_) => match save_lost_game_guest(result, safe_guest_id, current_day).await {
                Ok(_) => Json(json!({"message": "Game saved"})).into_response(),
                Err(err) => handle_error(err).into_response(),
            },
            None => match save_won_game_guest(result, safe_guest_id, current_day).await {
                Ok(_) => Json(json!({"message": "Game saved"})).into_response(),
                Err(err) => handle_error(err).into_response(),
            },
        },
    }
}

pub async fn get_guest_stats_api(headers: HeaderMap) -> impl IntoResponse {
    let header = headers.get("guest_id");
    if header.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Guest id not provided"})),
        )
            .into_response();
    }

    let safe_guest_id = headers
        .get("guest_id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    match get_guest_stats(safe_guest_id) {
        Ok(stats) => Json(json!(stats)).into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}

pub async fn get_guest_stats_day_api(
    headers: HeaderMap,
    Json(query): Json<DayQuery>,
) -> impl IntoResponse {
    let header = headers.get("guest_id");
    let day = query.day;
    if header.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Guest id not provided"})),
        )
            .into_response();
    }

    let safe_guest_id = headers
        .get("guest_id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    match get_guest_day_stats(safe_guest_id, day) {
        Ok(stats) => Json(json!(stats)).into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}


pub async fn get_guest_current_stats_day_api(
    headers: HeaderMap,
) -> impl IntoResponse {
    let header = headers.get("guest_id");
    let day = chrono::Local::now().format("%d/%m/%Y").to_string();
    if header.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Guest id not provided"})),
        )
            .into_response();
    }

    let safe_guest_id = headers
        .get("guest_id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    match get_guest_day_stats(safe_guest_id, day) {
        Ok(stats) => Json(json!(stats)).into_response(),
        Err(err) => handle_error(err).into_response(),
    }
}