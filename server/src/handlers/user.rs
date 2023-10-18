use axum::{extract, http};
use sqlx::SqlitePool;

use crate::dao::user::{CreateUser, User, UserDao};

pub async fn get_or_create(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Json(payload): extract::Json<CreateUser>,
) -> Result<(http::StatusCode, axum::Json<User>), http::StatusCode> {
    let res = UserDao::get_by_email(payload.email.clone(), &pool).await;

    match res {
        Ok(user) => Ok((http::StatusCode::OK, axum::Json(user))),
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let res = UserDao::create(payload, &pool).await;
                match res {
                    Ok(user) => Ok((http::StatusCode::CREATED, axum::Json(user))),
                    Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
            _ => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
