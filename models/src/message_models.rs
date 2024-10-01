use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Used to show a message to a user
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Message {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    pub content: String,
    pub sender_id: i32,
}

// Used to send a message
pub struct SendMessage {
    pub content: String,
}

// Returns the ID assigned by the DB when sending a message
pub struct NewMessage {
    pub id: i32,
}

// Used to delete a message
pub struct DeleteMessage {
    pub id: i32,
}
