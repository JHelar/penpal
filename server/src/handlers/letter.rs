use axum::{extract, http};
use sqlx::SqlitePool;

use crate::dao::{
    letter::{CreateLetter, Letter, LetterDao},
    sending_info::{SendingInfo, SendingInfoCreate, SendingInfoDao},
};

pub async fn create_letter(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<CreateLetter>,
) -> Result<(http::StatusCode, axum::Json<Letter>), http::StatusCode> {
    let res = LetterDao::create(payload, &pool).await;

    match res {
        Ok(letter) => Ok((http::StatusCode::CREATED, axum::Json(letter))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<CreateLetter>,
) -> Result<http::StatusCode, http::StatusCode> {
    let res = LetterDao::update(id, payload, &pool).await;

    match res {
        Ok(_) => Ok(http::StatusCode::OK),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<(http::StatusCode, axum::Json<Letter>), http::StatusCode> {
    let res = LetterDao::get_by_id(id, &pool).await;

    match res {
        Ok(letter) => Ok((http::StatusCode::OK, axum::Json(letter))),
        Err(error) => match error {
            sqlx::Error::RowNotFound => Err(http::StatusCode::NOT_FOUND),
            _ => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn delete_letter(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<http::StatusCode, http::StatusCode> {
    let delete_letter_res = LetterDao::delete(id, &pool).await;

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

    let get_letter_res = LetterDao::get_by_id(id, &pool).await;
    if let Ok(letter) = get_letter_res {
        if letter.sending_info_id == None {
            return Err(http::StatusCode::NOT_FOUND);
        }
        let payload =
            SendingInfoCreate::new(letter.id, "Stockholm".to_string(), "India".to_string(), now);

        let create_sending_info_res = SendingInfoDao::create(payload, &pool).await;

        match create_sending_info_res {
            Ok(sending_info) => {
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
                            println!("send_letter: Cannot update letter, letter does not exist");
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
    Err(http::StatusCode::NOT_FOUND)
}
