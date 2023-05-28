use std::env;

use axum::{response::IntoResponse, Json};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use hyper::{HeaderMap, StatusCode};
use jwt::{VerifyWithKey};
use serde_json::json;
use sha2::Sha256;
use std::{collections::BTreeMap};

use crate::{error_handler::MyError, models::token::DecodedToken};

pub fn decode_token(headers: HeaderMap) -> Result<DecodedToken, MyError>{
    dotenv().ok();
    match headers.get("Authorization"){
        Some(token) =>{
            let token = token.to_str().unwrap();
            let token = token.replace("Bearer ", "");
        
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

            let claims: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);
            match claims {
                Ok(claims) => {
                    let decoded_token = DecodedToken{
                        sub: claims.get("sub").unwrap().to_string(),
                        user: claims.get("user").unwrap().parse::<i32>().unwrap(),
                        iat: claims.get("iat").unwrap().to_string(),
                        exp: claims.get("exp").unwrap().to_string(),
                    };

                    if decoded_token.exp.parse::<i64>().unwrap() < chrono::Utc::now().timestamp(){
                        return Err(MyError{
                            message: String::from("Token expired"),
                            details: Some(String::from("Please login again")),
                            code: 401
                        })
                    }
                    return Ok(decoded_token);
                },
                Err(err) => {
                    return Err(MyError{
                        message: err.to_string(),
                        details: Some(String::from("Invalid token")),
                        code: 401
                    })
                }
            }
        },
        None => {
            Err(MyError{
                message: String::from("No token provided"),
                details: Some(String::from("Please login")),
                code: 401
            })
        },
    }
}

pub fn handle_error(err: MyError) -> impl IntoResponse {
    match err.code {
        400 => (StatusCode::BAD_REQUEST, Json(json!({"error": err.message}))),
        401 => (StatusCode::UNAUTHORIZED, Json(json!({"error": err.message}))),
        403 => (StatusCode::FORBIDDEN, Json(json!({"error": err.message}))),
        404 => (StatusCode::NOT_FOUND, Json(json!({"error": err.message}))),
        409 => (StatusCode::CONFLICT, Json(json!({"error": err.message}))),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.message})))
    }
}