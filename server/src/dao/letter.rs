use serde::Serialize;
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
    by_user_id: uuid::Uuid,
    to_user_id: uuid::Uuid,
    message: String,
}

impl LetterDao {
    pub async fn create(payload: CreateLetter, pool: SqlitePool) {
        let letter = Letter::new(payload.message, payload.to_user_id, payload.by_user_id);
        sqlx::query(
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
        .await
    }
}
