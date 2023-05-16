use axum::{response::IntoResponse, Json, extract::Query};

use serde_json::json;
use axum::http::HeaderMap;

use crate::{models::user::UserRequest, services::user_service};

use super::{utils::{decode_token, handle_error}, game_routes::DayQuery};


pub async fn sign_up_api(
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {

    let token = user_service::sign_up(user);

    match token {
        Ok(token) => Json(json!(token)).into_response(),
        Err(err) => handle_error(err).into_response()
    }
  
}

pub async fn login_api(Json(user): Json<UserRequest>) -> impl IntoResponse {

    let token = user_service::login(user);

    match token{
            Ok(token) => Json(json!(token)).into_response(),
            Err(err) => handle_error(err).into_response()
    }
}

pub async fn get_user_stats_api(
    headers: HeaderMap
) -> impl IntoResponse {    
    match decode_token(headers){
        Ok(token) => {
           match user_service::get_user_stats(token.user){
            Ok(stats) =>{
                 Json(json!(stats)).into_response()
            }
            Err(err) => handle_error(err).into_response()
           }
        },
        Err(err) => handle_error(err).into_response()
    }
}


pub async fn get_user_day_stats_api(
    headers: HeaderMap,
    Query (day_query): Query<DayQuery>
) -> impl IntoResponse {
    let day = day_query.day;
    match decode_token(headers){
        Ok(token) => {
           match user_service::get_user_day_stats(token.user, day){
            Ok(stats) =>{
                 Json(json!(stats)).into_response()
            }
            Err(err) => handle_error(err).into_response()
           }
        },
        Err(err) =>  handle_error(err).into_response()
    }  
}