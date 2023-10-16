use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

use super::letter::Letter;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    id: uuid::Uuid,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }
}

pub struct UserDao {}

impl UserDao {
    pub async fn create(pool: &SqlitePool) -> Result<User, sqlx::Error> {
        let user = User::new();

        let res = sqlx::query(
            r#"
            INSERT INTO users (id)
            VALUES ($1)    
            "#,
        )
        .bind(&user.id)
        .execute(pool)
        .await;

        match res {
            Ok(_) => Ok(user),
            Err(error) => {
                println!("UserDao::create: {:?}", error);
                Err(error)
            }
        }
    }

    pub async fn get_letters(
        id: uuid::Uuid,
        pool: &SqlitePool,
    ) -> Result<Vec<Letter>, sqlx::Error> {
        let res = sqlx::query_as::<_, Letter>(
            r#"
            SELECT * FROM letters
            WHERE by_user_id = $1
            "#,
        )
        .bind(id)
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
}
