use axum::{extract, http};
use sqlx::SqlitePool;

use crate::{
    dao::{
        letter::{CreateLetter, Letter, LetterDao},
        sending_info::{SendingInfo, SendingInfoCreate, SendingInfoDao},
    },
    middleware::authorize_current_user::CurrentUser,
};

pub async fn create_letter(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<CreateLetter>,
) -> Result<(http::StatusCode, axum::Json<Letter>), http::StatusCode> {
    let res = LetterDao::create(current_user.id, payload, &pool).await;

    match res {
        Ok(letter) => Ok((http::StatusCode::CREATED, axum::Json(letter))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_letter(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<CreateLetter>,
) -> Result<http::StatusCode, http::StatusCode> {
    let res = LetterDao::update(current_user.id, id, payload, &pool).await;

    match res {
        Ok(_) => Ok(http::StatusCode::OK),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_letter(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<(http::StatusCode, axum::Json<Letter>), http::StatusCode> {
    let res = LetterDao::get(current_user.id, id, &pool).await;

    match res {
        Ok(letter) => Ok((http::StatusCode::OK, axum::Json(letter))),
        Err(error) => match error {
            sqlx::Error::RowNotFound => Err(http::StatusCode::NOT_FOUND),
            _ => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn get_all_letters(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<Vec<Letter>>), http::StatusCode> {
    let res = LetterDao::get_all(current_user.id, &pool).await;

    match res {
        Ok(letters) => Ok((http::StatusCode::OK, axum::Json(letters))),
        Err(_) => Err(http::StatusCode::NOT_FOUND),
    }
}

pub async fn delete_letter(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<http::StatusCode, http::StatusCode> {
    if let Ok(letter) = LetterDao::get(current_user.id, id, &pool).await {
        if let Ok(transaction) = pool.begin().await {
            let delete_letter_res = LetterDao::delete(id, &pool).await;
            return match delete_letter_res {
                Ok(_) => {
                    if letter.sending_info_id == None {
                        return transaction.commit().await.map_or_else(
                            |error| {
                                println!("delete_letter: Transaction failed {:?}", error);
                                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                            },
                            |_| Ok(http::StatusCode::OK),
                        );
                    }

                    let delete_sending_info_res =
                        SendingInfoDao::delete(letter.sending_info_id.unwrap(), &pool).await;

                    match delete_sending_info_res {
                        Ok(_) => transaction.commit().await.map_or_else(
                            |error| {
                                println!("delete_letter: Transaction failed {:?}", error);
                                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                            },
                            |_| Ok(http::StatusCode::OK),
                        ),
                        Err(_) => transaction.rollback().await.map_or_else(
                            |error| {
                                println!("delete_letter: Rollback failed {:?}", error);
                                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                            },
                            |_| Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                        ),
                    }
                }
                Err(_) => transaction.rollback().await.map_or_else(
                    |error| {
                        println!("delete_letter: Rollback failed {:?}", error);
                        Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                    },
                    |_| Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                ),
            };
        }
    }
    Err(http::StatusCode::NOT_FOUND)
}

pub async fn send_letter(
    extract::Extension(current_user): extract::Extension<CurrentUser>,
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<(http::StatusCode, axum::Json<SendingInfo>), http::StatusCode> {
    let now = chrono::Utc::now();

    let get_letter_res = LetterDao::get(current_user.id, id, &pool).await;
    if let Ok(letter) = get_letter_res {
        if letter.sending_info_id.is_some() {
            println!("send_letter: Letter has allready been sent.");
            return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
        }

        if let Ok(transaction) = pool.begin().await {
            let payload = SendingInfoCreate::new(
                letter.id,
                "Stockholm".to_string(),
                "India".to_string(),
                now,
            );

            let create_sending_info_res = SendingInfoDao::create(payload, &pool).await;

            return match create_sending_info_res {
                Ok(sending_info) => {
                    let update_letter_res =
                        LetterDao::update_sending_info(id, sending_info.id, &pool).await;

                    match update_letter_res {
                        Ok(_) => transaction.commit().await.map_or_else(
                            |error| {
                                println!("send_letter: Transaction failed {:?}", error);
                                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                            },
                            |_| Ok((http::StatusCode::CREATED, axum::Json(sending_info))),
                        ),
                        Err(_) => transaction.rollback().await.map_or_else(
                            |_| Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                            |_| Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                        ),
                    }
                }
                Err(_) => transaction.rollback().await.map_or_else(
                    |error| {
                        println!("send_letter: Rollback failed {:?}", error);
                        Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                    },
                    |_| Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                ),
            };
        }
    }
    Err(http::StatusCode::NOT_FOUND)
}
