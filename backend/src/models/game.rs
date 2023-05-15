use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};
use crate::schema::user_day_stats;



#[derive(Queryable, Serialize, Deserialize)]
pub struct Game {
    id: i32,
    board: String,
    day: String,
    initial_position: String,
}

#[derive(Deserialize, Serialize)]
pub struct GameResult{
    pub uncovered: String,
    pub exploded: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = user_day_stats)]
pub struct UserDayStats {
    pub user_id: i32,
    pub day: String,
    pub status: String,
    pub board: String,
    pub last_move: Option<String>,
}


#[derive(Deserialize,Queryable)]
pub struct DayStat{
    pub day: String,
    pub total_games: i32,
    pub total_wins: i32,
    pub aggregated_board_stats: String,
}

