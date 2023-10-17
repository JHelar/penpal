use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize, FromRow)]
pub struct Letter {
    pub id: uuid::Uuid,
    pub message: String,
    pub to_user_id: uuid::Uuid,
    pub by_user_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub sending_info_id: Option<uuid::Uuid>,
}

impl Letter {
    pub fn new(message: String, to_user_id: uuid::Uuid, by_user_id: uuid::Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            message,
            to_user_id,
            by_user_id,
            created_at: now,
            updated_at: now,
            sending_info_id: None,
        }
    }
}

pub struct LetterDao {}

#[derive(Debug, Deserialize)]
pub struct CreateLetter {
    to_user_id: uuid::Uuid,
    message: String,
}

impl LetterDao {
    pub async fn get(
        by_user_id: uuid::Uuid,
        id: uuid::Uuid,
        pool: &SqlitePool,
    ) -> Result<Letter, sqlx::Error> {
        let res = sqlx::query_as::<_, Letter>(
            r#"
            SELECT * FROM letters
            WHERE id = $1 AND by_user_id = $2
            "#,
        )
        .bind(id)
        .bind(by_user_id)
        .fetch_one(pool)
        .await;

        match res {
            Ok(letter) => Ok(letter),
            Err(error) => {
                println!("LetterDao::get_by_id: {:?}", error);
                Err(error)
            }
        }
    }
    pub async fn get_all(
        by_user_id: uuid::Uuid,
        pool: &SqlitePool,
    ) -> Result<Vec<Letter>, sqlx::Error> {
        let res = sqlx::query_as::<_, Letter>(
            r#"
            SELECT * FROM letters
            WHERE by_user_id = $1
            "#,
        )
        .bind(by_user_id)
        .fetch_all(pool)
        .await;

        match res {
            Ok(letters) => Ok(letters),
            Err(error) => {
                println!("UserDao::get_letters: {:?}", error);
                Err(error)
            }
        }
    }
    pub async fn create(
        by_user_id: uuid::Uuid,
        payload: CreateLetter,
        pool: &SqlitePool,
    ) -> Result<Letter, sqlx::Error> {
        let letter = Letter::new(payload.message, payload.to_user_id, by_user_id);
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
        .execute(pool)
        .await;

        match res {
            Ok(_) => Ok(letter),
            Err(error) => {
                println!("LetterDao::create: {:?}", error);
                Err(error)
            }
        }
    }
    pub async fn update(
        by_user_id: uuid::Uuid,
        id: uuid::Uuid,
        payload: CreateLetter,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now();
        let res = sqlx::query(
            r#"
            UPDATE letters
            SET message = $1, to_user_id = $2, updated_at = $3
            WHERE id = $4 AND by_user_id = $5 AND sending_info_id IS NULL
            "#,
        )
        .bind(&payload.message)
        .bind(&payload.to_user_id)
        .bind(now)
        .bind(id)
        .bind(by_user_id)
        .execute(pool)
        .await;

        match res {
            Ok(query_result) => match query_result.rows_affected() {
                0 => {
                    println!("LetterDao::update: No letter to update.");
                    Err(sqlx::Error::RowNotFound)
                }
                _ => Ok(()),
            },
            Err(error) => {
                println!("LetterDao::update: {:?}", error);
                Err(error)
            }
        }
    }
    pub async fn delete(id: uuid::Uuid, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let res = sqlx::query(
            r#"
            DELETE FROM letters
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("LetterDao::delete: {:?}", error);
                Err(error)
            }
        }
    }
    pub async fn update_sending_info(
        id: uuid::Uuid,
        sending_info_id: uuid::Uuid,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let res = sqlx::query(
            r#"
                UPDATE letters
                SET sending_info_id = $1
                WHERE id = $2
                "#,
        )
        .bind(sending_info_id)
        .bind(id)
        .execute(pool)
        .await;

        match res {
            Ok(query_result) => match query_result.rows_affected() {
                0 => {
                    println!("LetterDao::update_sending: Letter not found");
                    Err(sqlx::Error::RowNotFound)
                }
                _ => Ok(()),
            },
            Err(error) => {
                println!("LetterDao::update_sending: Letter not found");
                Err(error)
            }
        }
    }
}
