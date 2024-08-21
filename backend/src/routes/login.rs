use axum::{extract::State, routing::post, Json, Router};
use jwt_simple::{
    claims::Claims,
    prelude::{Duration, MACLike},
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::query_as;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{
    error::{Error, Result},
    models::User,
    AppState,
};

use super::{JWTContent, AUTH_TOKEN};

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", post(login_post))
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login_post(
    State(AppState { pool, jwt_key }): State<AppState>,
    cookies: Cookies,
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

    let jwt_content = JWTContent { id: result.id };

    let claims = Claims::with_custom_claims(jwt_content, Duration::from_mins(20));
    let token = jwt_key.authenticate(claims).map_err(|_| Error::LoginFail)?;

    cookies.add(Cookie::new(AUTH_TOKEN, token.clone()));

    Ok(Json(json!({
        "success": true,
        "token": token
    })))
}
