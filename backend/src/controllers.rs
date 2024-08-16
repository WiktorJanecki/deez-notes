use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jwt_simple::prelude::UnixTimeStamp;
use sqlx::{query, query_as, Execute, PgPool, QueryBuilder};

use crate::{
    error::{Error, Result},
    models::{Note, NoteForCreate, NoteForEdit},
    session::Session,
};

pub async fn create_note(note_fc: NoteForCreate, session: Session, pool: PgPool) -> Result<Note> {
    let time_created: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("machine time backwards")
        .as_secs() as i64;
    let result = query_as!(
        Note,
        "INSERT INTO notes (creator_id, title, content, time_created, time_edited) VALUES ($1, $2, $3, $4, $4) RETURNING *",
        session.id,
        note_fc.title,
        note_fc.content,
        time_created,
    ).fetch_one(&pool).await.map_err(|_| Error::SQLFail)?;
    Ok(result)
}

pub async fn change_note(
    id: i32,
    note_fe: NoteForEdit,
    _session: Session,
    pool: PgPool,
) -> Result<Note> {
    let time_edited: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("machine time backwards")
        .as_secs() as i64;
    let mut query = QueryBuilder::new("UPDATE notes SET");
    if let Some(title) = note_fe.title {
        query.push("title = ").push_bind(title);
    }
    if let Some(content) = note_fe.content {
        query.push("content= ").push_bind(content);
    }
    query.push("WHERE id = ").push_bind(id).push("RETURNING *");
    let result: Note = query
        .build_query_as()
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::SQLFail)?;
    Ok(result)
}
pub async fn get_notes(session: Session, pool: PgPool) -> Result<Vec<Note>> {
    let result = query_as!(
        Note,
        "SELECT * FROM notes WHERE creator_id = $1",
        session.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| Error::SQLFail)?;
    Ok(result)
}

pub async fn get_note(id: i32, _session: Session, pool: PgPool) -> Result<Note> {
    let result = query_as!(Note, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::SQLFail)?;
    Ok(result)
}
