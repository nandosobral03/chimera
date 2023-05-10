pub mod user;

use diesel::{prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Game {
    id: i32,
    board: String,
    day: String,
    initial_position: String,
}