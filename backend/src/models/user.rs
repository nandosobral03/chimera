use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize};

#[derive(Queryable, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct UserCreate {
    pub email: String,
    pub password_hash: String,
    pub salt: String,

}

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub enum UserType{
    Player,
    Guest
}



#[derive(Deserialize,Queryable)]
pub struct DayStat{
    pub day: String,
    pub total_games: i32,
    pub total_wins: i32,
    pub aggregated_board_stats: String,
}

