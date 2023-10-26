mod dao;
mod handlers;
mod middleware;

use std::env;

use axum::routing::{delete, get, post, put, Router};
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use tower_http::cors::{Any, CorsLayer};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    let options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    MIGRATOR.run(&pool).await?;

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let user_routes = Router::new()
        .route("/", post(handlers::user::update))
        .route(
            "/random_recipient",
            get(handlers::user::get_random_recipient),
        )
        .route("/letter", get(handlers::letter::get_all_letters))
        .route("/letter", post(handlers::letter::create_letter))
        .route("/letter/:id", get(handlers::letter::get_letter))
        .route("/letter/:id", put(handlers::letter::update_letter))
        .route("/letter/:id", delete(handlers::letter::delete_letter))
        .route(
            "/letter/:id/info",
            get(handlers::sending_info::get_sending_info),
        )
        .route("/letter/:id/send", put(handlers::letter::send_letter))
        .route_layer(axum::middleware::from_fn(middleware::auth));

    let app = Router::new()
        .nest("/me", user_routes)
        // Non authenticated routes bellow
        .route("/", get(handlers::hello_world))
        .route("/signIn", post(handlers::user::get_or_create))
        .layer(cors)
        .with_state(pool);

    println!("Starting server at: http://{}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
