use std::collections::HashMap;

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
    pub flags: String,
    pub time_taken: i32,
}

#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = user_day_stats)]
pub struct UserDayStats {
    pub user_id: i32,
    pub day: String,
    pub status: String,
    pub board: String,
    pub last_move: Option<String>,
    pub flags: String,
    pub time_taken: i32,
}


#[derive(Deserialize,Queryable, Serialize, Debug)]
pub struct DayStat{
    pub day: String,
    pub total_games: i32,
    pub total_wins: i32,
    pub aggregated_board_stats: String,
}

#[derive(Serialize)]
pub struct DayStateResponse{
    pub day: String,
    pub total_games: i32,
    pub total_wins: i32,
    pub aggregated_board_stats: HashMap<String, i32>
}


#[derive(Serialize)]
pub struct DayLeaderboardResponse{
    pub day: String,
    pub leaderboard: Vec<LeaderboardEntry>
}

#[derive(Serialize)]
pub struct LeaderboardEntry{
    pub username: String,
    pub time_taken: i32
}

#[derive(Serialize)]
pub struct AllTimeLeaderboardResponse{
    pub users: Vec<AllTimeLeaderboardEntry>,
    pub guests: Vec<AllTimeLeaderboardEntry>
}

#[derive(Serialize)]
pub struct AllTimeLeaderboardEntry{
    pub username: String,
    pub total_wins: i32,
    pub total_games: i32,
    pub win_streak: i32,
}

