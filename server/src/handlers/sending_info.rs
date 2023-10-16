use axum::{extract, http};
use sqlx::SqlitePool;

use crate::dao::sending_info::{SendingInfo, SendingInfoDao};

pub async fn get_sending_info(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(letter_id): extract::Path<uuid::Uuid>,
) -> Result<(http::StatusCode, axum::Json<SendingInfo>), http::StatusCode> {
    let res = SendingInfoDao::get_by_letter_id(letter_id, &pool).await;

    match res {
        Ok(sending_info) => Ok((http::StatusCode::OK, axum::Json(sending_info))),
        Err(error) => match error {
            sqlx::Error::RowNotFound => Err(http::StatusCode::NOT_FOUND),
            _ => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
