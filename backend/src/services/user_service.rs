use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, Connection};
use dotenvy::dotenv;
use rand::RngCore;

use crate::{
    error_handler::MyError,
    models::{
        token::Token,
        user::{User, UserCreate, UserRequest, DayStat}, game::{GameResult},
    }
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
                email: user.email.clone(),
                password_hash: hash.format_for_version(bcrypt::Version::TwoB),
                salt: hex::encode(salt),
            };
            let mut conn = crate::database::establish_connection();
            match diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)
            {
                Ok(_) =>{
                    let new_user_id = users::table.filter(users::email.eq(&user.email)).select(users::id).first::<i32>(&mut conn).unwrap();
                    generate_token(user.email,new_user_id)
                },
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
    }
}

pub fn login(user: UserRequest) -> Result<Token, MyError> {
    use crate::schema::users::dsl::*;
    let mut conn = crate::database::establish_connection();
    let result = users.filter(email.eq(&user.email)).first::<User>(&mut conn);
    match result {
        Ok(u) => {
            let mut new_salt = [0u8; 16];
            new_salt.copy_from_slice(&hex::decode(u.salt).unwrap());
            let new_hash = bcrypt::hash_with_salt(user.password, 4, new_salt);
            match new_hash {
                Ok(hash) => {
                    if hash.format_for_version(bcrypt::Version::TwoB) == u.password_hash {
                        let token = generate_token(user.email, u.id);
                        match token {
                            Ok(token) => {
                                return Ok(token);
                            }
                            Err(err) => {
                                return Err(MyError {
                                    message: err.message,
                                    code: 400,
                                })
                            }
                        }
                    } else {
                        return Err(MyError {
                            message: String::from("Invalid password"),
                            code: 401,
                        });
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
        Err(err) => Err(MyError {
            message: err.to_string(),
            code: 400,
        }),
    }
}

pub fn generate_token(email: String, new_user_id: i32) -> Result<Token, MyError> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", email);
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
            code: 409,
        });
    } else {
        match diesel::insert_into(user_day_stats)
            .values((
                user_id.eq(userid),
                day.eq(&result_day),
                status.eq("won"),
                board.eq(result.uncovered),
                last_move.eq(None::<String>),
            ))
            .execute(&mut conn)
        {
            Ok(_) => {
                record_stats(result_day, true, None)?;
                return Ok(());
            }
            Err(err) => {
                return Err(MyError {
                    message: err.to_string(),
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
            code: 409,
        });
    } else {
        match diesel::insert_into(user_day_stats)
            .values((
                user_id.eq(userid),
                day.eq(&result_day),
                status.eq("lost"),
                board.eq(result.uncovered),
                last_move.eq(&result.exploded),
            ))
            .execute(&mut conn)
        {
            Ok(_) => record_stats(result_day, false, result.exploded),
            Err(err) => {
                return Err(MyError {
                    message: err.to_string(),
                    code: 400,
                })
            }
        }
    }
}

fn record_stats(result_day: String, won: bool, last_move_played: Option<String>) -> Result<(), MyError> {
    let mut conn = crate::database::establish_connection();
    let exists = diesel::select(diesel::dsl::exists(
        crate::schema::day_stats::dsl::day_stats.filter(crate::schema::day_stats::dsl::day.eq(&result_day))
    )).get_result::<bool>(&mut conn).unwrap();



    let insert_result = diesel::insert_into(crate::schema::day_stats::table)
    .values((
        crate::schema::day_stats::dsl::day.eq(&result_day),
        crate::schema::day_stats::dsl::total_games.eq(1),
        crate::schema::day_stats::dsl::total_wins.eq(if won {0} else {0}),
        crate::schema::day_stats::dsl::aggregated_board_stats.eq("{}"),
    ))
    .execute(&mut conn);

    if !exists && insert_result.is_err() {
        return Err(MyError {
            message: String::from("Error inserting day stat"),
            code: 400,
        });
    }
    


    if won {
        match conn.transaction::<_, diesel::result::Error, _>(|conn| {
            use crate::schema::day_stats::dsl::*;
            let existing_day_stat: DayStat = day_stats.first(conn)?;
            let updated_games_won = existing_day_stat.total_wins + 1;
            let updated_total_games = existing_day_stat.total_games + 1;
            diesel::update(day_stats)
                .filter(day.eq(&result_day))
                .set((
                    total_wins.eq(updated_games_won),
                    total_games.eq(updated_total_games),
                ))
                .execute(conn)
        }) {
            Ok(_) => Ok(()),
            Err(err) => Err(MyError {
                message: err.to_string(),
                code: 400,
            }),
        }
    } else{
        match last_move_played {
            None => {
                return Err(MyError {
                    message: String::from("Last move played is required in losing games"),
                    code: 400,
                })
            },
            Some(val) =>{ 
                match conn.transaction::<_, diesel::result::Error, _>(|conn| {
                    use crate::schema::day_stats::dsl::*;
                    let existing_day_stat: DayStat = day_stats.first(conn)?;
                    let updated_total_games = existing_day_stat.total_games + 1;
                    let current_day_stat = day_stats.filter(day.eq(&result_day)).first::<DayStat>(conn)?;
                    let mut board_stats: BTreeMap<String, i32> = serde_json::from_str(&current_day_stat.aggregated_board_stats).unwrap_or(BTreeMap::new());
                    if board_stats.contains_key(&val) {
                        let updated_board_stat = board_stats.get(&val).unwrap() + 1;
                        board_stats.insert(val, updated_board_stat);
                    }
                    
                    diesel::update(day_stats)
                        .filter(day.eq(&result_day))
                        .set((
                            total_games.eq(updated_total_games),
                            aggregated_board_stats.eq(serde_json::to_string(&board_stats).unwrap())
                        ))
                        .execute(conn)
                }) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(MyError {
                        message: err.to_string(),
                        code: 400,
                    }),
                }
            }
        }

        
    }


   
}
