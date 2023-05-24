use crate::{
    error_handler::MyError,
    models::{
        game::GameResult,
        guest::{ Guest, GuestDayStats},
    },
    services::game_service::{record_stats, validate_game_result},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub async fn save_won_game_guest(
    result: GameResult,
    param_guest_id: String,
    result_day: String,
) -> Result<(), MyError> {
    validate_game_result(&result)?;
    use crate::schema::guest_day_stats::dsl::*;
    use crate::schema::guests::dsl::*;
    let mut conn = crate::database::establish_connection();
    let guest_exists = guests
        .filter(id.eq(&param_guest_id))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    let guest_day_count = guest_day_stats
        .filter(guest_id.eq(&param_guest_id))
        .filter(day.eq(&result_day))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    if guest_day_count != 0 {
        return Err(MyError {
            message: String::from("User already played this day"),
            code: 409,
        });
    }

    if guest_exists == 0 {
        use crate::schema::guests::dsl::*;
        let inserted = diesel::insert_into(guests)
            .values((id.eq(&param_guest_id), total_games.eq(0), total_wins.eq(0)))
            .execute(&mut conn);
        if inserted.is_err() {
            return Err(MyError {
                message: String::from("Error inserting guest"),
                code: 400,
            });
        }
    }

    let insert_res = diesel::insert_into(guest_day_stats)
        .values((
            guest_id.eq(&param_guest_id),
            day.eq(&result_day),
            status.eq("won"),
            board.eq(result.uncovered),
            flags.eq(result.flags),
            time_taken.eq(result.time_taken),
            last_move.eq(None::<String>),
        ))
        .execute(&mut conn);

    if insert_res.is_err() {
        return Err(MyError {
            message: String::from("Error inserting guest day stats"),
            code: 400,
        });
    }

    record_guest_stats(param_guest_id, true)?;
    record_stats(result_day, true, None)?;
    Ok(())
}

pub async fn save_lost_game_guest(
    result: GameResult,
    param_guest_id: String,
    result_day: String,
) -> Result<(), MyError> {
    validate_game_result(&result)?;

    use crate::schema::guest_day_stats::dsl::*;
    use crate::schema::guests::dsl::*;
    let mut conn = crate::database::establish_connection();

    let guest_exists = guests
        .filter(id.eq(&param_guest_id))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    let guest_day_count = guest_day_stats
        .filter(guest_id.eq(&param_guest_id))
        .filter(day.eq(&result_day))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    if guest_day_count != 0 {
        return Err(MyError {
            message: String::from("User already played this day"),
            code: 409,
        });
    }

    if guest_exists == 0 {
        use crate::schema::guests::dsl::*;
        let inserted = diesel::insert_into(guests)
            .values((id.eq(&param_guest_id), total_games.eq(0), total_wins.eq(0)))
            .execute(&mut conn);
        if inserted.is_err() {
            return Err(MyError {
                message: String::from("Error inserting guest"),
                code: 400,
            });
        }
    }

    let insert_res = diesel::insert_into(guest_day_stats)
        .values((
            guest_id.eq(&param_guest_id),
            day.eq(&result_day),
            status.eq("lost"),
            board.eq(result.uncovered),
            flags.eq(result.flags),
            time_taken.eq(result.time_taken),
            last_move.eq(&result.exploded),
        ))
        .execute(&mut conn);

    if insert_res.is_err() {
        return Err(MyError {
            message: String::from("Error inserting guest day stats"),
            code: 400,
        });
    }

    record_guest_stats(param_guest_id, false)?;
    record_stats(result_day, false, result.exploded)?;
    Ok(())
}

fn record_guest_stats(guest_id_param: String, won: bool) -> Result<(), MyError> {
    use crate::schema::guests::dsl::*;
    use crate::schema::guest_day_stats::dsl::*;
    let mut conn = crate::database::establish_connection();

    let guest_result = guests.filter(id.eq(&guest_id_param)).first::<Guest>(&mut conn);
    let yesterday = (chrono::Local::now().date_naive() - chrono::Duration::days(1))
        .format("%d/%m/%Y")
        .to_string();
    match guest_result {
        Ok(guest) => {
            let new_total_games = guest.total_games + 1;
            let new_total_wins = if won {
                guest.total_wins + 1
            } else {
                guest.total_wins
            };
            let last_win: Result<GuestDayStats, _> = guest_day_stats
                .filter(guest_id.eq(&guest_id_param))
                .filter(status.eq("won"))
                .order_by(day.desc())
                .first::<GuestDayStats>(&mut conn);
            let update_res = diesel::update(guests)
                .filter(id.eq(&guest_id_param))
                .set((
                    total_games.eq(new_total_games),
                    total_wins.eq(new_total_wins),
                    win_streak.eq(match last_win {
                        Ok(last_win) => {
                            if last_win.day == yesterday {
                                guest.win_streak + (won as i32)
                            } else {
                                won as i32
                            }
                        }
                        Err(_) => won as i32,
                    }),
                ))
                .execute(&mut conn);
            if update_res.is_err() {
                Err(MyError {
                    message: String::from("Error updating guest stats"),
                    code: 400,
                })
            } else {
                Ok(())
            }
        }
        Err(_) => Err(MyError {
            message: String::from("Error getting guest stats"),
            code: 400,
        }),
    }
}


pub fn get_guest_stats(param_guest_id: String) -> Result<Guest, MyError> {
    use crate::schema::guests::dsl::*;
    let mut conn = crate::database::establish_connection();

    let guest_result = guests.filter(id.eq(&param_guest_id)).first::<Guest>(&mut conn);

    match guest_result {
        Ok(guest) => Ok(guest),
        Err(_) => Err(MyError {
            message: String::from("Error getting guest stats"),
            code: 400,
        }),
    }
}



pub fn get_guest_day_stats(param_guest_id: String, param_day: String) -> Result<GuestDayStats, MyError> {
    use crate::schema::guests::dsl::*;
    use crate::schema::guest_day_stats::dsl::*;
    let mut conn = crate::database::establish_connection();

    let guest_result = guests.filter(id.eq(&param_guest_id)).first::<Guest>(&mut conn);

    match guest_result {
        Ok(_guest) => {
            let guest_day_result = guest_day_stats
                .filter(guest_id.eq(&param_guest_id))
                .filter(day.eq(&param_day))
                .first::<GuestDayStats>(&mut conn);
            match guest_day_result {
                Ok(guest_day) => Ok(guest_day),
                Err(_) => Err(MyError {
                    message: String::from("Error getting guest day stats"),
                    code: 400,
                }),
            }
        }
        Err(_) => Err(MyError {
            message: String::from("Guest not found"),
            code: 404,
        }),
    }
}


