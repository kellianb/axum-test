use chrono::Utc;
use models::message_models::*;
use sqlx::{query, query_as, Error, Pool, Postgres};

pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Message>, Error> {
    query_as!(
        Message,
        "SELECT id, sent_at, content, sender_id FROM messages WHERE deleted = FALSE;"
    )
    .fetch_all(pool)
    .await
}

pub async fn get(id: i32, pool: &Pool<Postgres>) -> Result<Message, Error> {
    query_as!(
        Message,
        "SELECT id, sent_at, content, sender_id
        FROM messages
        WHERE deleted = FALSE
            AND id = $1;",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create(
    message_data: SendMessage,
    sender_id: i32,
    pool: &Pool<Postgres>,
) -> Result<NewMessage, Error> {
    query_as!(
        NewMessage,
        r#"INSERT INTO messages (id, sent_at, content, sender_id, deleted)
        VALUES (DEFAULT, $1, $2, $3, DEFAULT)
        RETURNING id"#,
        Utc::now().naive_utc(),
        message_data.content,
        sender_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(id: i32, pool: &Pool<Postgres>) -> Result<u64, Error> {
    Ok(
        query!("UPDATE messages SET deleted = TRUE WHERE id = $1;", id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}
