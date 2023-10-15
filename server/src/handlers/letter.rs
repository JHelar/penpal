use axum::{extract, http};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::dao::{letter::Letter, sending_info::SendingInfo};

#[derive(Debug, Deserialize)]
pub struct CreateLetter {
    by_user_id: uuid::Uuid,
    to_user_id: uuid::Uuid,
    message: String,
}

pub async fn create_letter(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<CreateLetter>,
) -> Result<(http::StatusCode, axum::Json<Letter>), http::StatusCode> {
    let letter = Letter::new(payload.message, payload.to_user_id, payload.by_user_id);
    let res = sqlx::query(
        r#"
        INSERT INTO letters (id, message, to_user_id, by_user_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&letter.id)
    .bind(&letter.message)
    .bind(&letter.to_user_id)
    .bind(&letter.by_user_id)
    .bind(&letter.created_at)
    .bind(&letter.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(letter))),
        Err(error) => {
            println!("create_letter: {:?}", error);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<CreateLetter>,
) -> Result<http::StatusCode, http::StatusCode> {
    let now = chrono::Utc::now();

    let res = sqlx::query(
        r#"
        UPDATE letters
        SET message = $1, to_user_id = $2, by_user_id = $3, updated_at = $4
        WHERE id = $5 AND sending_info_id IS NULL
        "#,
    )
    .bind(&payload.message)
    .bind(&payload.to_user_id)
    .bind(&payload.by_user_id)
    .bind(now)
    .bind(id)
    .execute(&pool)
    .await;

    match res {
        Ok(query_result) => match query_result.rows_affected() {
            0 => {
                println!("update_letter: {:?}", query_result);
                Err(http::StatusCode::NOT_FOUND)
            }
            _ => Ok(http::StatusCode::OK),
        },
        Err(error) => {
            println!("update_letter: {:?}", error);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<(http::StatusCode, axum::Json<Letter>), http::StatusCode> {
    let res = sqlx::query_as::<_, Letter>(
        r#"
        SELECT * FROM letters
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(letter) => Ok((http::StatusCode::OK, axum::Json(letter))),
        Err(error) => {
            println!("get_letter: {:?}", error);
            match error {
                sqlx::Error::RowNotFound => Err(http::StatusCode::NOT_FOUND),
                _ => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

pub async fn delete_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<http::StatusCode, http::StatusCode> {
    let delete_letter_res = sqlx::query(
        r#"
        DELETE FROM letters
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await;

    match delete_letter_res {
        Ok(_) => {
            let delete_sending_info_res = sqlx::query(
                r#"
                DELETE FROM sending_info
                WHERE letter_id = $1
                "#,
            )
            .bind(id)
            .execute(&pool)
            .await;

            match delete_sending_info_res {
                Ok(_) => Ok(http::StatusCode::OK),
                Err(error) => {
                    println!("delete_letter: {:?}", error);
                    Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(error) => {
            println!("delete_letter: {:?}", error);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn send_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<(http::StatusCode, axum::Json<SendingInfo>), http::StatusCode> {
    let now = chrono::Utc::now();

    let res = sqlx::query_as::<_, Letter>(
        r#"
        SELECT * FROM letters
        WHERE id = $1 AND sending_info_id IS NULL
        "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(letter) => {
            let sending_info =
                SendingInfo::new(letter.id, "Stockholm".to_string(), "India".to_string(), now);

            let create_sending_info_res = sqlx::query(
                r#"
                INSERT INTO sending_infos (id, letter_id, sent_at, received_at, from_loc, to_loc, eta)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#
            )
            .bind(&sending_info.id)
            .bind(&sending_info.letter_id)
            .bind(&sending_info.sent_at)
            .bind(&sending_info.received_at)
            .bind(&sending_info.from_loc)
            .bind(&sending_info.to_loc)
            .bind(&sending_info.eta)
            .execute(&pool)
            .await;

            match create_sending_info_res {
                Ok(_) => {
                    let update_letter_res = sqlx::query(
                        r#"
                        UPDATE letters
                        SET sending_info_id = $1
                        WHERE id = $2
                        "#,
                    )
                    .bind(&sending_info.id)
                    .bind(id)
                    .execute(&pool)
                    .await;

                    match update_letter_res {
                        Ok(result) => match result.rows_affected() {
                            0 => {
                                println!(
                                    "send_letter: Cannot update letter, letter does not exist"
                                );
                                Err(http::StatusCode::NOT_FOUND)
                            }
                            _ => Ok((http::StatusCode::CREATED, axum::Json(sending_info))),
                        },
                        Err(error) => {
                            println!("send_letter: {:?}", error);
                            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                Err(error) => {
                    println!("send_letter: {:?}", error);
                    Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(error) => {
            println!("send_letter: {:?}", error);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
