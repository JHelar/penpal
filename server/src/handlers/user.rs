use axum::{extract, http};
use sqlx::SqlitePool;

use crate::dao::user::{User, UserDao};

pub async fn create(
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<User>), http::StatusCode> {
    let res = UserDao::create(&pool).await;

    match res {
        Ok(user) => Ok((http::StatusCode::CREATED, axum::Json(user))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
