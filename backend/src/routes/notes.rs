use axum::{extract::State, routing::get, Json, Router};

use crate::{
    controllers,
    error::Result,
    models::{Note, NoteForCreate},
    session::Session,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/notes", get(get_notes).post(post_notes))
}

async fn post_notes(
    State(AppState { pool, .. }): State<AppState>,
    payload: Json<NoteForCreate>,
) -> Result<Json<Note>> {
    let result = controllers::create_note(payload.0, Session { id: 0 }, pool).await?;
    Ok(Json(result))
}

async fn get_notes(State(AppState { pool, .. }): State<AppState>) -> Result<Json<Vec<Note>>> {
    let result = controllers::get_notes(Session { id: 0 }, pool).await?;
    Ok(Json(result))
}
