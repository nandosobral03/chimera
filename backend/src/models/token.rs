use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

pub struct DecodedToken{
    pub sub: String,
    pub user: i32,
    pub iat: String,
    pub exp: String,

}

