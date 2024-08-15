use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use dotenv::dotenv;
use models::User;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool};

mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    // setup db

    let pool = PgPoolOptions::new()
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // routing
    let app = Router::new()
        .merge(routes::login::routes())
        .with_state(pool);

    // starting server

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
