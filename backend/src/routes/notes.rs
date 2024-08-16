use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    controllers::{self, change_note},
    error::Result,
    models::{Note, NoteForCreate, NoteForEdit},
    session::Session,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/notes", get(get_notes).post(post_notes))
        .route(
            "/notes/:id",
            get(get_note).delete(delete_note).put(update_note),
        )
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

async fn get_note(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Note>> {
    let result = controllers::get_note(id, session, pool).await?;
    Ok(Json(result))
}

async fn delete_note(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
) -> Result<()> {
    controllers::delete_note(id, session, pool).await?;
    Ok(())
}

async fn update_note(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
    payload: Json<NoteForEdit>,
) -> Result<Json<Note>> {
    let result = change_note(id, payload.0, session, pool).await?;
    Ok(Json(result))
}
