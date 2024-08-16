use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{query, query_as, PgPool};

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
        session.id(),
        note_fc.title,
        note_fc.content,
        time_created,
    ).fetch_one(&pool).await.map_err(|_| Error::SQLFail)?;
    Ok(result)
}

pub async fn change_note(
    id: i32,
    note_fe: NoteForEdit,
    session: Session,
    pool: PgPool,
) -> Result<Note> {
    let time_edited: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("machine time backwards")
        .as_secs() as i64;

    let result = query_as!(
        Note,
        "
            UPDATE notes SET
                time_edited = $1,
                title = COALESCE($2, title),
                content = COALESCE($3, content)
            WHERE 
                creator_id = $4 AND id = $5
            RETURNING *
        ",
        time_edited,
        note_fe.title,
        note_fe.content,
        session.id(),
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| Error::SQLFail)?;

    Ok(result)
}
pub async fn get_notes(session: Session, pool: PgPool) -> Result<Vec<Note>> {
    let result = query_as!(
        Note,
        "SELECT * FROM notes WHERE creator_id = $1",
        session.id()
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| Error::SQLFail)?;
    Ok(result)
}

pub async fn get_note(id: i32, session: Session, pool: PgPool) -> Result<Note> {
    let result = query_as!(Note, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::SQLFail)?;
    if result.creator_id != session.id() {
        return Err(Error::AuthNoAccess);
    }
    Ok(result)
}
pub async fn delete_note(id: i32, session: Session, pool: PgPool) -> Result<()> {
    query!(
        "DELETE FROM notes WHERE creator_id = $1 and id = $2",
        session.id(),
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| Error::SQLFail)?;
    Ok(())
}
