use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use crate::{models::{game::GameResult, guest}, services::guest_service::{save_lost_game_guest, save_won_game_guest}};
use axum::http::HeaderMap;


pub async fn post_game_result_guest(
    headers: HeaderMap,
    Json(result): Json<GameResult>,
) -> impl IntoResponse {
    let current_day = chrono::Local::now().format("%d/%m/%Y").to_string();

    let header = headers.get("guest_id");
    if header.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Guest id not provided"}))).into_response();
    }

    match headers.get("guest_id").unwrap().to_str().unwrap().to_string() {
        safe_guest_id => {
            match result.exploded{
                Some(_) => {
                    match save_lost_game_guest(result, safe_guest_id, current_day).await{
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
                    match save_won_game_guest(result, safe_guest_id, current_day).await{
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
        }
        _ => (StatusCode::BAD_REQUEST, Json(json!({"error": "Guest id not provided"}))).into_response(),
    }
}
