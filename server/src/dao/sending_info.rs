use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize, FromRow)]
pub struct SendingInfo {
    pub id: uuid::Uuid,
    pub letter_id: uuid::Uuid,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    pub from_loc: String,
    pub to_loc: String,
    pub eta: chrono::DateTime<chrono::Utc>,
}

impl SendingInfo {
    pub fn new(
        letter_id: uuid::Uuid,
        from_loc: String,
        to_loc: String,
        eta: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            letter_id,
            sent_at: chrono::Utc::now(),
            received_at: None,
            from_loc,
            to_loc,
            eta,
        }
    }
}

pub struct SendingInfoDao {}

#[derive(Debug)]
pub struct SendingInfoCreate {
    letter_id: uuid::Uuid,
    from_loc: String,
    to_loc: String,
    eta: chrono::DateTime<chrono::Utc>,
}

impl SendingInfoCreate {
    pub fn new(
        letter_id: uuid::Uuid,
        from_loc: String,
        to_loc: String,
        eta: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            letter_id,
            from_loc,
            to_loc,
            eta,
        }
    }
}

impl SendingInfoDao {
    pub async fn create(
        payload: SendingInfoCreate,
        pool: &SqlitePool,
    ) -> Result<SendingInfo, sqlx::Error> {
        let sending_info = SendingInfo::new(
            payload.letter_id,
            payload.from_loc,
            payload.to_loc,
            payload.eta,
        );
        let res = sqlx::query(
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
                .execute(pool)
                .await;
        match res {
            Ok(_) => Ok(sending_info),
            Err(error) => {
                println!("SendingInfoDao::create: {:?}", error);
                Err(error)
            }
        }
    }
}
