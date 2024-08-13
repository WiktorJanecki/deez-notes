use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool};
use tracing::Level;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // setup db

    let pool = PgPoolOptions::new()
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // routing

    let app = Router::new()
        .route("/user", get(user).post(post_user))
        .with_state(pool);

    // starting server

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn user(State(pool): State<PgPool>) -> (StatusCode, Json<Vec<User>>) {
    let row: Vec<User> = query_as!(User, "SELECT * from users")
        .fetch_all(&pool)
        .await
        .unwrap();
    (StatusCode::OK, Json(row))
}

#[derive(Deserialize)]
struct AddInput {
    username: String,
    password: String,
}
async fn post_user(State(pool): State<PgPool>, payload: Json<AddInput>) -> StatusCode {
    query!(
        "INSERT INTO users (username, password) VALUES ($1,$2)",
        payload.username,
        payload.password
    )
    .execute(&pool)
    .await
    .unwrap();
    StatusCode::OK
}
