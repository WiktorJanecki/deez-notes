use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

// Note

#[derive(FromRow, Serialize)]
pub struct Note {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub content: String,
    pub time_created: i64,
    pub time_edited: i64,
}

#[derive(Deserialize)]
pub struct NoteForCreate {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct NoteForEdit {
    pub title: Option<String>,
    pub content: Option<String>,
}
