use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Used to show a message to a user
#[derive(Serialize, Deserialize, Debug, FromRow)]
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
