use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    id: uuid::Uuid,
    email: String,
}

impl User {
    pub fn new(email: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            email,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
}

pub struct UserDao {}

impl UserDao {
    pub async fn create(payload: CreateUser, pool: &SqlitePool) -> Result<User, sqlx::Error> {
        let user = User::new(payload.email);

        let res = sqlx::query(
            r#"
            INSERT INTO users (id, email)
            VALUES ($1, $2)    
            "#,
        )
        .bind(&user.id)
        .bind(&user.email)
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

    pub async fn get_by_email(email: String, pool: &SqlitePool) -> Result<User, sqlx::Error> {
        let res = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE email = $1   
            "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(error) => {
                println!("UserDao::get_by_email: {:?}", error);
                Err(error)
            }
        }
    }
}
