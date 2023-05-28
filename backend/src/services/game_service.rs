use std::collections::BTreeMap;

use diesel::prelude::*;
use crate::{error_handler::MyError,database::establish_connection, models::{game::{Game, DayStat, GameResult, DayStateResponse, DayLeaderboardResponse, UserDayStats, LeaderboardEntry, AllTimeLeaderboardEntry, AllTimeLeaderboardResponse}, user::{User, UserStats}, guest::{Guest}}};

pub fn get_game_by_day(day_to_find: &str) -> Result<Game, MyError> {
    validate_day(day_to_find)?;
    use crate::schema::games::dsl::*;
    let connection = &mut establish_connection();
    let results: Result<Game, _> = games
        .filter(day.eq(day_to_find))
        .first::<Game>(connection);
    match results {
        Ok(game) => Ok(game),
        Err(_) => Err(MyError{
            message: String::from("Game not found"),
            details: None,
            code: 404
        })
    }
}





fn validate_day(day_to_find: &str) -> Result<(), MyError> {
    let parts = day_to_find.split("/").collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err(MyError{
            message: String::from("Invalid date. Please use the format dd/mm/yyyy"),
            details: None,
            code: 400
        });
    }
    let day = parts[0].parse::<i32>().unwrap_or(0);
    let month = parts[1].parse::<i32>().unwrap_or(0);
    let year = parts[2].parse::<i32>().unwrap_or(0);

    if day < 1 || day > 31 {
        return Err(
            MyError {
                message: String::from("Invalid day. Please use a day between 1 and 31"),
                details: None,
                code: 400
            }
        );
    }

    if month < 1 || month > 12 {
        return Err(
            MyError {
                message: String::from("Invalid month. Please use a month between 1 and 12"),
                details: None,
                code: 400
            }
        );
    }

    if year < 2023 || year > 2025 {
        return Err(
            MyError {
                message: String::from("Invalid year. Please use a year between 2022 and 2025"),
                details: None,
                code: 400
            }
        );
    }

    Ok(())
}



pub fn record_stats(result_day: String, won: bool, last_move_played: Option<String>) -> Result<(), MyError> {
    let mut conn = crate::database::establish_connection();
    let exists = diesel::select(diesel::dsl::exists(
        crate::schema::day_stats::dsl::day_stats.filter(crate::schema::day_stats::dsl::day.eq(&result_day))
    )).get_result::<bool>(&mut conn).unwrap();



    let insert_result = diesel::insert_into(crate::schema::day_stats::table)
    .values((
        crate::schema::day_stats::dsl::day.eq(&result_day),
        crate::schema::day_stats::dsl::total_games.eq(0),
        crate::schema::day_stats::dsl::total_wins.eq(0),
        crate::schema::day_stats::dsl::aggregated_board_stats.eq("{}"),
    ))
    .execute(&mut conn);

    if !exists && insert_result.is_err() {
        return Err(MyError {
            message: String::from("Error inserting day stat"),
            details: None,
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
                message: String::from("Error recording stats"),
                details: Some(err.to_string()),
                code: 400,
            }),
        }
    } else{
        match last_move_played {
            None => {
                return Err(MyError {
                    message: String::from("Last move played is required in losing games"),
                    details: None,
                    code: 400,
                })
            },
            Some(val) =>{ 
                match conn.transaction::<_, diesel::result::Error, _>(|conn| {
                    use crate::schema::day_stats::dsl::*;
                    let current_day_stat = day_stats.filter(day.eq(&result_day)).first::<DayStat>(conn)?;
                    let updated_total_games = current_day_stat.total_games + 1;
                    let mut board_stats: BTreeMap<String, i32> = serde_json::from_str(&current_day_stat.aggregated_board_stats).unwrap_or(BTreeMap::new());
                    if board_stats.contains_key(&val) {
                        let updated_board_stat = board_stats.get(&val).unwrap() + 1;
                        board_stats.insert(val, updated_board_stat);
                    }else{
                        board_stats.insert(val, 1);
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
                        message: String::from("Error recording stats"),
                        details: Some(err.to_string()),
                        code: 400,
                    }),
                }
            }
        }
    }
   
}


pub fn get_day_stats(day_to_find: &str) -> Result<DayStateResponse, MyError> {
    validate_day(day_to_find)?;
    use crate::schema::day_stats::dsl::*;
    let connection = &mut establish_connection();
    let results: Result<DayStat, _> = day_stats
        .filter(day.eq(day_to_find))
        .first::<DayStat>(connection);
    match results {
        Ok(day_stat) => Ok(
            DayStateResponse{
                day: day_stat.day,
                total_games: day_stat.total_games,
                total_wins: day_stat.total_wins,
                aggregated_board_stats: serde_json::from_str(&day_stat.aggregated_board_stats).unwrap()
            }
        ),
        Err(_) => Err(MyError{
            message: String::from("Day stat not found"),
            details: None,
            code: 404
        })
    }
}


pub fn validate_game_result(result: &GameResult)-> Result<(), MyError>{
    if let Some(exploded) = &result.exploded {
        validate_cell(exploded)?;
    }

    let uncovered = 
        result.uncovered.split(",").collect::<Vec<&str>>()
        .iter()
        .map(|&x | validate_cell(x))
        .collect::<Result<Vec<()>, MyError>>();
    if uncovered.is_err() {
        return Err(MyError{
            message: String::from("Invalid cell. Please use the format x:y"),
            details: None,
            code: 400
        })
    }

    if result.flags.len() == 0 {
        return Ok(())
    }
    let flags = 
        result.flags.split(",").collect::<Vec<&str>>()
        .iter()
        .map(|&x | validate_cell(x))
        .collect::<Result<Vec<()>, MyError>>();
    if flags.is_err() {
        return Err(MyError{
            message: String::from("Invalid cell. Please use the format x:y"),
            details: None,
            code: 400
        })
    }



    Ok(())
}


pub fn get_day_leaderboards(day_to_find: &str) -> Result<DayLeaderboardResponse, MyError> {
    use crate::schema::user_day_stats::dsl::*;
    use crate::schema::users::dsl::*;
    validate_day(day_to_find)?;
    let connection = &mut establish_connection();
    let results: Result<Vec<UserDayStats>, _> = user_day_stats
        .filter(crate::schema::user_day_stats::dsl::day.eq(day_to_find))
        .filter(crate::schema::user_day_stats::dsl::status.eq("won"))
        .order(crate::schema::user_day_stats::dsl::time_taken.asc())
        .limit(20)
        .load::<UserDayStats>(connection);
    
    match results {
        Ok(day_stat) => {
            let leaderboard: Vec<LeaderboardEntry> = day_stat.iter().map(|x| {
                let user = users.filter(crate::schema::users::dsl::id.eq(x.user_id)).first::<User>(connection);
                match user {
                    Ok(user) => {
                        LeaderboardEntry{
                            username: user.username.clone(),
                            time_taken: x.time_taken
                        }
                    },
                    Err(_) => {
                        LeaderboardEntry{
                            username: String::from("Unknown"),
                            time_taken: x.time_taken
                        }
                    }
                }
            }).collect();
            Ok(DayLeaderboardResponse { day: day_to_find.to_string(), leaderboard })
        },
        Err(_) => Err(MyError{
                message: String::from("Day stat not found"),
                details: None,
                code: 404
            })
    }
   
}


pub fn get_all_time_stats() -> Result<AllTimeLeaderboardResponse, MyError> {
    let connection = &mut establish_connection();
    let user_all_time_stats = {
        use crate::schema::user_stats::dsl::*;
        let result = user_stats.order_by((win_streak.desc(), total_wins.desc())).limit(20).load::<UserStats>(connection);
        match result {
            Ok(stats) => stats,
            Err(_) => Vec::new()
        }
    };

    let guest_all_time_stats = {
        use crate::schema::guests::dsl::*;
        let result = guests.order(total_games.desc()).limit(20).load::<Guest>(connection);
        match result {
            Ok(stats) => stats,
            Err(_) => Vec::new()
        }
    };    

    let user_list = user_all_time_stats.iter().map(|x| {
        use crate::schema::users::dsl::*;
        AllTimeLeaderboardEntry{
            username: {
                let user = users.filter(crate::schema::users::dsl::id.eq(x.user_id)).first::<User>(connection);
                match user {
                    Ok(user) => user.username.clone(),
                    Err(_) => String::from("Unknown")
                }
            },
            win_streak: x.win_streak,
            total_games: x.total_games,
            total_wins: x.total_wins,
        }}).collect::<Vec<AllTimeLeaderboardEntry>>();
   
   let guest_list = guest_all_time_stats.iter().map(|x| {
        AllTimeLeaderboardEntry{
            username: x.id.clone(),
            win_streak: x.win_streak,
            total_games: x.total_games,
            total_wins: x.total_wins,
        }}).collect::<Vec<AllTimeLeaderboardEntry>>();

    Ok(AllTimeLeaderboardResponse{
        users: user_list,
        guests: guest_list
    })
}



fn validate_cell(cell: &str) -> Result<(), MyError> {
    let parts = cell.split(":").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(MyError{
            message: String::from("Invalid cell. Please use the format x:y"),
            details: None,
            code: 400
        })
    }

    let x = parts[0].parse::<i32>().unwrap_or(0);
    let y = parts[1].parse::<i32>().unwrap_or(0);

    if x < 0 || x > 15 {
        return Err(MyError{
            message: String::from("Invalid cell value. Please use a value between 0 and 15 for x"),
            details: None,
            code: 400
        })
    }

    if y < 0 || y > 29 {
        return Err(MyError{
            message: String::from("Invalid cell value. Please use a value between 0 and 29 for y"),
            details: None,
            code: 400
        })
    }
    Ok(())
}


