use axum::{extract, http};
use sqlx::SqlitePool;

use crate::{
    dao::user::{CreateUser, UpdateUser, User, UserDao},
    middleware::authorize_current_user::CurrentUser,
};

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

pub async fn update(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
    extract::Json(payload): extract::Json<UpdateUser>,
) -> Result<http::StatusCode, http::StatusCode> {
    let res = UserDao::update(current_user.id, payload, &pool).await;

    match res {
        Ok(_) => Ok(http::StatusCode::ACCEPTED),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
