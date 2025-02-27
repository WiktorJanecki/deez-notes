use serde::{Deserialize, Serialize};

pub mod login;
pub mod notes;

pub const AUTH_TOKEN: &str = "AUTH_TOKEN";

#[derive(Deserialize, Serialize)]
pub struct JWTContent {
    pub id: i32,
}
