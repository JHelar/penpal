mod dao;

mod handlers;
use std::env;

use axum::routing::{delete, get, post, put, Router};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    let options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    let app = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/letter", post(handlers::letter::create_letter))
        .route("/letter/:id", get(handlers::letter::get_letter))
        .route("/letter/:id", put(handlers::letter::update_letter))
        .route("/letter/:id", delete(handlers::letter::delete_letter))
        .route("/letter/:id/send", put(handlers::letter::send_letter))
        .with_state(pool);

    println!("Starting server at: http://{}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
