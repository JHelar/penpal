use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    email: String,
    username: Option<String>,
    pub display_name: Option<String>,
    pub profile_image: Option<String>,
    is_initialized: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {}

impl User {
    pub fn new(email: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            email,
            display_name: None,
            profile_image: None,
            username: None,
            is_initialized: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub display_name: String,
    pub profile_image: String,
}

pub struct UserDao {}

impl UserDao {
    pub async fn create(payload: CreateUser, pool: &SqlitePool) -> Result<User, sqlx::Error> {
        let user = User::new(payload.email);

        let res = sqlx::query(
            r#"
            INSERT INTO users (id, email, created_at, updated_at, is_initialized)
            VALUES ($1, $2, $3, $4, $5)    
            "#,
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .bind(false)
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

    pub async fn get_all_other_users(
        user_id: uuid::Uuid,
        pool: &SqlitePool,
    ) -> Result<Vec<User>, sqlx::Error> {
        let res = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE id != $1 AND is_initialized = 1 
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await;

        match res {
            Ok(users) => Ok(users),
            Err(error) => {
                println!("UserDao::get_all_other_users: {:?}", error);
                Err(error)
            }
        }
    }

    pub async fn update(
        user_id: uuid::Uuid,
        payload: UpdateUser,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now();
        let res = sqlx::query(
            r#"
            UPDATE users
            SET username = $1, display_name = $2, profile_image = $3, is_initialized = $4, updated_at = $5
            WHERE id = $6
            "#,
        )
        .bind(&payload.username)
        .bind(&payload.display_name)
        .bind(&payload.profile_image)
        .bind(true)
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await;

        match res {
            Ok(query_result) => match query_result.rows_affected() {
                0 => {
                    println!("UserDao::update: No user to update.");
                    Err(sqlx::Error::RowNotFound)
                }
                _ => Ok(()),
            },
            Err(error) => {
                println!("UserDao::update: update {:?}", error);
                Err(error)
            }
        }
    }
}
