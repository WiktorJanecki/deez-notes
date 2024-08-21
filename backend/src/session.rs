use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use jwt_simple::{
    common::VerificationOptions,
    prelude::{Duration, MACLike},
};
use tower_cookies::Cookies;
use tracing::warn;

use crate::{
    error::{Error, Result},
    routes::{JWTContent, AUTH_TOKEN},
    AppState,
};

pub struct Session {
    id: i32,
}
impl Session {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Session
where
    AppState: FromRef<S>,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let cookies = parts
            .extract::<Cookies>()
            .await
            .map_err(|_| Error::AuthFailMissingCookie)?;
        let cookie = cookies
            .get(AUTH_TOKEN)
            .ok_or(Error::AuthFailMissingCookie)?;
        let state = AppState::from_ref(state);
        let token = cookie.value();

        let token_claims = state
            .jwt_key
            .verify_token::<JWTContent>(
                token,
                Some(VerificationOptions {
                    time_tolerance: Some(Duration::from_hours(2)),
                    ..Default::default()
                }),
            )
            .map_err(|_| Error::AuthFailBadToken)?;
        let session_id = token_claims.custom.id;
        Ok(Session::new(session_id))
    }
}
