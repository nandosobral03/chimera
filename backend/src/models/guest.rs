use diesel::{Insertable, Queryable};
use serde::Deserialize;
use serde::Serialize;

use crate::schema::guests;
use crate::schema::guest_day_stats;





#[derive(Insertable, Deserialize, Queryable, Serialize)]
#[diesel(table_name = guests)]
pub struct Guest {
    pub id: String,
    pub total_games: i32,
    pub total_wins: i32,

}

#[derive(Insertable, Deserialize, Queryable, Serialize)]
#[diesel(table_name = guest_day_stats)]
pub struct GuestDayStats{
    pub guest_id: String,
    pub day: String,
    pub status: String,
    pub board: String,
    pub last_move: Option<String>,
}