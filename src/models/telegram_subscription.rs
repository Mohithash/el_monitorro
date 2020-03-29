use chrono::{DateTime, Utc};

#[derive(Queryable, Debug)]
pub struct TelegramSubscription {
    pub chat_id: i64,
    pub feed_id: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
