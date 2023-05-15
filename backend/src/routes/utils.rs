use std::env;

use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use hyper::HeaderMap;
use jwt::{VerifyWithKey};
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
                            code: 401
                        })
                    }
                    return Ok(decoded_token);
                },
                Err(err) => {
                    return Err(MyError{
                        message: err.to_string(),
                        code: 401
                    })
                }
            }
        },
        None => {
            Err(MyError{
                message: String::from("No token provided"),
                code: 401
            })
        },
    }
}