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
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    payload: Json<NoteForCreate>,
) -> Result<Json<Note>> {
    let result = controllers::create_note(payload.0, session, pool).await?;
    Ok(Json(result))
}

async fn get_notes(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
) -> Result<Json<Vec<Note>>> {
    let result = controllers::get_notes(session, pool).await?;
    Ok(Json(result))
}
