use diesel::{Insertable, Queryable};
use serde::Deserialize;

use crate::schema::guests;





#[derive(Insertable, Deserialize, Queryable)]
#[diesel(table_name = guests)]
pub struct Guest {
    pub id: String,
    pub total_games: i32,
    pub total_wins: i32,

}