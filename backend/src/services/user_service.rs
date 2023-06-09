use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use rand::RngCore;

use crate::{
    error_handler::MyError,
    models::{
        token::Token,
        user::{User, UserCreate, UserRequest, UserStats}, game::{GameResult, UserDayStats},
    }, services::game_service::{record_stats, validate_game_result}
};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::{collections::BTreeMap, env};



pub fn sign_up(user: UserRequest) -> Result<Token, MyError> {
    use crate::schema::users;
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    let password_hash = bcrypt::hash_with_salt(user.password, 4, salt);
    match password_hash {
        Ok(hash) => {
             let new_user = UserCreate {
                username: user.username.clone(),
                password_hash: hash.format_for_version(bcrypt::Version::TwoB),
                salt: hex::encode(salt),
            };
            let mut conn = crate::database::establish_connection();
            match diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)
            {
                Ok(_) =>{
                    let new_user_id = users::table.filter(users::username.eq(&user.username)).select(users::id).first::<i32>(&mut conn).unwrap();
                    generate_token(user.username,new_user_id)
                },
                Err(err) => {
                    match err {
                        diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation,_) => {
                            return Err(MyError {
                                message: String::from("Username already exists"),
                                details: None,
                                code: 409,
                            })
                        },
                        _ => {
                            return Err(MyError {
                                message: String::from("Error inserting user"),
                                details: None,                                
                                code: 400,
                            })
                        }
                    }
                }
            }


        }
        Err(err) => {
            return Err(MyError {
                message: String::from("Error hashing password"),
                details: Some(err.to_string()),
                code: 400,
            })
        }
    }
}

pub fn login(user: UserRequest) -> Result<Token, MyError> {
    use crate::schema::users::dsl::*;
    let mut conn = crate::database::establish_connection();
    let result = users.filter(username.eq(&user.username)).first::<User>(&mut conn);
    match result {
        Ok(u) => {
            let mut new_salt = [0u8; 16];
            new_salt.copy_from_slice(&hex::decode(u.salt).unwrap());
            let new_hash = bcrypt::hash_with_salt(user.password, 4, new_salt);
            match new_hash {
                Ok(hash) => {
                    if hash.format_for_version(bcrypt::Version::TwoB) == u.password_hash {
                        let token = generate_token(user.username, u.id)?;
                        return Ok(token);
                    } else {
                        return Err(MyError {
                            message: String::from("Invalid password"),
                            details: None,
                            code: 401,
                        });
                    }
                }
                Err(err) => {
                    return Err(MyError {
                        message: String::from("Error hashing password"),
                        details: Some(err.to_string()),
                        code: 400,
                    })
                }
            }
        }
        Err(err) => Err(MyError {
            message: String::from("Invalid username"),
            details: Some(err.to_string()),
            code: 400,
        }),
    }
}

pub fn generate_token(username: String, new_user_id: i32) -> Result<Token, MyError> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", username);
    claims.insert("user", new_user_id.to_string());
    claims.insert("iat", chrono::Utc::now().timestamp().to_string());
    claims.insert(
        "exp",
        (chrono::Utc::now() + chrono::Duration::days(30))
            .timestamp()
            .to_string(),
    );
    let token_str = claims.sign_with_key(&key).unwrap();
    let token = Token { token: token_str };
    Ok(token)
}

pub async fn save_won_game(
    result: GameResult,
    userid: i32,
    result_day: String,
) -> Result<(), MyError> {
    use crate::schema::user_day_stats::dsl::*;
    validate_game_result(&result)?;
    let mut conn = crate::database::establish_connection();

    let user_day_count = user_day_stats
        .filter(user_id.eq(&userid))
        .filter(day.eq(&result_day))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    if user_day_count != 0 {
        return Err(MyError {
            message: String::from("User already played this day"),
            details: None,
            code: 409,
        });
    } else {
        match diesel::insert_into(user_day_stats)
            .values((
                user_id.eq(userid),
                day.eq(&result_day),
                status.eq("won"),
                board.eq(result.uncovered),
                flags.eq(result.flags),
                time_taken.eq(result.time_taken),
                last_move.eq(None::<String>),
            ))
            .execute(&mut conn)
        {
            Ok(_) => {
                record_user_stats(userid, true)?;
                record_stats(result_day, true, None)?;
                return Ok(());
            }
            Err(err) => {
                return Err(MyError {
                    message: String::from("Error inserting user day stats"),
                    details: Some(err.to_string()),
                    code: 400,
                })
            }
        }
    }
}

pub async fn save_lost_game(
    result: GameResult,
    userid: i32,
    result_day: String,
) -> Result<(), MyError> {
    use crate::schema::user_day_stats::dsl::*;
    validate_game_result(&result)?;
    let mut conn = crate::database::establish_connection();

    let user_day_count = user_day_stats
        .filter(user_id.eq(&userid))
        .filter(day.eq(&result_day))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    if user_day_count != 0 {
        return Err(MyError {
            message: String::from("User already played this day"),
            details: None,
            code: 409,
        });
    } else {
        match diesel::insert_into(user_day_stats)
            .values((
                user_id.eq(userid),
                day.eq(&result_day),
                status.eq("lost"),
                flags.eq(result.flags),
                board.eq(result.uncovered),
                last_move.eq(&result.exploded),
                time_taken.eq(result.time_taken),
            ))
            .execute(&mut conn)
        {
            Ok(_) => {
                record_user_stats(userid, false)?;
                record_stats(result_day, false, result.exploded)
            },
            Err(err) => {
                return Err(MyError {
                    message: err.to_string(),
                    details: None,
                    code: 400,
                })
            }
        }
    }
}


fn record_user_stats(userid: i32, won:bool) -> Result<(), MyError>{
    use crate::schema::user_stats::dsl::*;
    let mut conn = crate::database::establish_connection();
    let exists = user_stats.filter(user_id.eq(&userid)).count().get_result::<i64>(&mut conn).unwrap() != 0;

    if !exists {
        let insert_result = diesel::insert_into(crate::schema::user_stats::table)
        .values((
            user_id.eq(&userid),
            total_games.eq(0),
            total_wins.eq(0),
            win_streak.eq(0),
        ))
        .execute(&mut conn);

        if !exists && insert_result.is_err() {
            return Err(MyError {
                message: String::from("Error inserting day stat"),
                details: Some(insert_result.unwrap_err().to_string()),
                code: 400,
            });
        }
    }

    
    let mut stats = user_stats
        .filter(user_id.eq(&userid))
        .first::<UserStats>(&mut conn)
        .unwrap();
    stats.total_games += 1;
    if won {
        stats.total_wins += 1;
    }

    match diesel::update(user_stats).filter(user_id.eq(&userid))
        .set((
            total_games.eq(stats.total_games),
            total_wins.eq(stats.total_wins),
            win_streak.eq(if won { stats.win_streak + 1 } else { 0 })
        ))
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(MyError {
                message: err.to_string(),
                details: None,
                code: 400,
            })
        }
    }
}


pub fn get_user_stats(userid: i32) -> Result<UserStats, MyError> {
    use crate::schema::user_stats::dsl::*;
    let mut conn = crate::database::establish_connection();
    let stats = user_stats
        .filter(user_id.eq(&userid))
        .first::<UserStats>(&mut conn)
        .unwrap();
    Ok(stats)
}

pub fn get_user_day_stats(userid: i32, day_param: String) -> Result<UserDayStats, MyError> {
    use crate::schema::user_day_stats::dsl::*;
    let mut conn = crate::database::establish_connection();
    match user_day_stats
        .filter(user_id.eq(&userid))
        .filter(day.eq(&day_param))
        .first::<UserDayStats>(&mut conn){ 
            Ok(stats) => Ok(stats),
            Err(err) => {
                return Err(MyError {
                    message: err.to_string(),
                    details: None,
                    code: 404,
                })
            }
        }
}

