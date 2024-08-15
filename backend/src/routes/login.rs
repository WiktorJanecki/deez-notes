use std::borrow::Borrow;

use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{query, query_as, PgPool, Pool, Postgres};
use tracing::info;

use crate::{
    error::{Error, Result},
    models::User,
};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/api/login", post(login_post))
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login_post(
    State(pool): State<PgPool>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
    let username = payload.username.to_owned();
    let result = query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::LoginFail)?;
    if result.password != payload.password {
        info!("Bad password!");
        return Err(Error::LoginFail);
    }
    Ok(Json(json!({
        "success": true
    })))
}
