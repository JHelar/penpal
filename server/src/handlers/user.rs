use axum::{extract, http};
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::{
    dao::user::{CreateUser, UpdateUser, User, UserDao},
    middleware::authorize_current_user::CurrentUser,
};

const DEFAULT_AVATAR_PROFILE_IMAGE: &str = "https://as1.ftcdn.net/v2/jpg/03/53/11/00/1000_F_353110097_nbpmfn9iHlxef4EDIhXB1tdTD0lcWhG9.jpg";

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

#[derive(Debug, Serialize)]
pub struct Recipient {
    id: uuid::Uuid,
    display_name: String,
    profile_image: String,
}

impl Recipient {
    fn from_user(user: &User) -> Self {
        Self {
            display_name: user
                .display_name
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            id: user.id,
            profile_image: user
                .profile_image
                .clone()
                .unwrap_or_else(|| DEFAULT_AVATAR_PROFILE_IMAGE.to_string()),
        }
    }
}

pub async fn get_random_recipient(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<Recipient>), http::StatusCode> {
    let res = UserDao::get_all_other_users(current_user.id, &pool).await;

    match res {
        Ok(users) => {
            let user = if let Some(user) = users.choose(&mut thread_rng()) {
                user
            } else {
                return Err(http::StatusCode::NO_CONTENT);
            };

            let recipient = Recipient::from_user(user);
            Ok((http::StatusCode::OK, axum::Json(recipient)))
        }
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_recipient(
    extract::Path(id): extract::Path<uuid::Uuid>,
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<Recipient>), http::StatusCode> {
    let res = UserDao::get_by_id(id, &pool).await;

    match res {
        Ok(user) => {
            let recipient = Recipient::from_user(&user);

            Ok((http::StatusCode::OK, axum::Json(recipient)))
        }
        Err(_) => Err(http::StatusCode::NO_CONTENT),
    }
}
