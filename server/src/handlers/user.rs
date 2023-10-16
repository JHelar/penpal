use axum::{extract, http};
use sqlx::SqlitePool;

use crate::{
    dao::{
        letter::Letter,
        user::{User, UserDao},
    },
    middleware::CurrentUser,
};

pub async fn create(
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<User>), http::StatusCode> {
    let res = UserDao::create(&pool).await;

    match res {
        Ok(user) => Ok((http::StatusCode::CREATED, axum::Json(user))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_letter_by_user(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<Vec<Letter>>), http::StatusCode> {
    let res = UserDao::get_letters(current_user.id, &pool).await;

    match res {
        Ok(letters) => Ok((http::StatusCode::OK, axum::Json(letters))),
        Err(_) => Err(http::StatusCode::NOT_FOUND),
    }
}
