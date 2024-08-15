use axum::Router;
use dotenv::dotenv;
use jwt_simple::prelude::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_cookies::CookieManagerLayer;

mod error;
mod models;
mod routes;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    jwt_key: HS256Key,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    // setup db

    let pool = PgPoolOptions::new()
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    //jwt key
    let jwt_key = HS256Key::generate();

    let state = AppState { pool, jwt_key };

    // routing
    let app = Router::new()
        .merge(routes::login::routes())
        .layer(CookieManagerLayer::new())
        .with_state(state);

    // starting server

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
