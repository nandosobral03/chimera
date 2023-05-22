use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UserCreate {
    pub username: String,
    pub password_hash: String,
    pub salt: String,

}

#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Queryable, Serialize)]
pub struct UserStats{
    pub user_id: i32,
    pub total_games: i32,
    pub total_wins: i32,    
}

