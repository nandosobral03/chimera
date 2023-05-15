use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::{models::user::UserRequest, services::user_service};


pub async fn sign_up_api(
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {

    let token = user_service::sign_up(user);

    match token {
        Ok(token) => Json(json!(token)).into_response(),
        Err(err) =>  {
            match err.code {
                400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
                409 => (StatusCode::CONFLICT, Json(json!({"error": err.message}))).into_response(),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
            }
        }
    }
  
}

pub async fn login_api(Json(user): Json<UserRequest>) -> impl IntoResponse {

    let token = user_service::login(user);

    match token{
            Ok(token) => Json(json!(token)).into_response(),
            Err(err) =>  {
                match err.code {
                    400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))).into_response(),
                    401 => (StatusCode::UNAUTHORIZED, Json(json!({"error": err.message}))).into_response(),
                    409 => (StatusCode::CONFLICT, Json(json!({"error": err.message}))).into_response(),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message}))).into_response()
                }
            }
    }
}
