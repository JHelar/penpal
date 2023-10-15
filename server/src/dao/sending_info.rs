use serde::Serialize;
use sqlx::FromRow;

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
