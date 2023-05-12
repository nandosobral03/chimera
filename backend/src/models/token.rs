use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}