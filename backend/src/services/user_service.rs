use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use dotenvy::dotenv;
use rand::RngCore;

use crate::{
    error_handler::MyError,
    models::{
        token::Token,
        user::{User, UserCreate, UserRequest},
    },
};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::{collections::BTreeMap, env};

pub fn sign_up(user: UserRequest) -> Result<Token, MyError> {
    use crate::schema::users;
    let mut salt = [0u8;16];
    rand::thread_rng().fill_bytes(&mut salt);
    let password_hash = bcrypt::hash_with_salt(user.password, 4, salt);
    match password_hash {
        Ok(hash) => {
            println!("{}", hash.get_salt());
            let new_user = UserCreate {
                email: user.email.clone(),
                password_hash: hash.format_for_version(bcrypt::Version::TwoB),
                salt: hex::encode(salt),
            };

            let token = generate_token(user.email);
            match token {
                Ok(token) => {
                    let mut conn = crate::database::establish_connection();
                    match diesel::insert_into(users::table)
                        .values(&new_user)
                        .execute(&mut conn)
                    {
                        Ok(_) => return Ok(token),
                        Err(err) => {
                            return Err(MyError {
                                message: err.to_string(),
                                code: 400,
                            })
                        }
                    }
                }
                Err(err) => {
                    return Err(MyError {
                        message: err.to_string(),
                        code: 400,
                    })
                }
            };
        }
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
                code: 400,
            })
        }
    }
}


pub fn login( user: UserRequest ) -> Result<Token, MyError> {
    use crate::schema::users::dsl::*;
    let mut conn = crate::database::establish_connection();
    let result = users.filter(
        email.eq(&user.email)
    ).first::<User>(&mut conn);
    match result{
        Ok(u) => {

            //Salt is stored salt.iter().map(|b| format!("{:02x}", b)).collect::<String>(),
            let mut new_salt = [0u8;16];
            new_salt.copy_from_slice(&hex::decode(u.salt).unwrap());
            let new_hash = bcrypt::hash_with_salt(user.password, 4, new_salt);
            match new_hash{
                Ok(hash) => {
                    if hash.format_for_version(bcrypt::Version::TwoB) == u.password_hash{
                        let token = generate_token(user.email);
                        match token{
                            Ok(token) => {
                                return Ok(token);
                            }
                            Err(err) => {
                                return Err(MyError {
                                    message: err.to_string(),
                                    code: 400,
                                })
                            }
                        }
                    }else{
                        return Err(MyError {
                            message: String::from("Invalid password"),
                            code: 401,
                        })
                    }
                }
                Err(err) => {
                    return Err(MyError {
                        message: err.to_string(),
                        code: 400,
                    })
                }
            }
        }
        Err(err) => {
            Err(MyError {
                message: err.to_string(),
                code: 400,
            })
        }
    }
        
        
}   


pub fn generate_token(email: String) -> Result<Token, jwt::Error> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", email);
    claims.insert("iat", chrono::Utc::now().timestamp().to_string());
    claims.insert(
        "exp",
        (chrono::Utc::now() + chrono::Duration::days(30))
            .timestamp()
            .to_string(),
    );
    let token_str = claims.sign_with_key(&key)?;
    let token = Token { token: token_str };
    Ok(token)
}
