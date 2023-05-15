use diesel::prelude::*;
use crate::{error_handler::MyError,database::establish_connection, models::game::Game};

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
            code: 404
        })
    }
}





fn validate_day(day_to_find: &str) -> Result<(), MyError> {
    let parts = day_to_find.split("/").collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err(MyError{
            message: String::from("Invalid date. Please use the format dd/mm/yyyy"),
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
                code: 400
            }
        );
    }

    if month < 1 || month > 12 {
        return Err(
            MyError {
                message: String::from("Invalid month. Please use a month between 1 and 12"),
                code: 400
            }
        );
    }

    if year < 2023 || year > 2025 {
        return Err(
            MyError {
                message: String::from("Invalid year. Please use a year between 2022 and 2025"),
                code: 400
            }
        );
    }

    Ok(())
}


