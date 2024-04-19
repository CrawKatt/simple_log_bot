use tokio::time::{Duration, Instant, sleep_until};
use chrono::Local;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, MessageId, UserId};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb::Result as SurrealResult;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[allow(deprecated)]
pub fn clean_database_loop() {
    tokio::spawn(async {
        loop {
            let now = Local::now();
            let midnight = now + chrono::Duration::days(1);
            let midnight = midnight.date().and_hms(0, 0, 0);
            let duration_until_midnight = (midnight - now).to_std().unwrap_or_else(|why| {
                eprintln!("Could not get duration until midnight: {why}");
                Duration::from_secs(60 * 60 * 24)
            });

            sleep_until(Instant::now() + duration_until_midnight).await; // 24 horas (60 * 60 * 24)

            DB.use_ns("discord-namespace").use_db("discord").await.unwrap_or_else(|why| {
                eprintln!("Could not use namespace: {why}");
                panic!("Could not use namespace: {why}");
            });

            DB.delete("messages").await.unwrap_or_else(|why| -> Vec<MessageData> {
                eprintln!("Could not delete messages: {why}");
                panic!("Could not delete messages: {why}");
            });

            DB.delete("audio").await.unwrap_or_else(|why| -> Vec<MessageData> {
                eprintln!("Could not delete audio: {why}");
                panic!("Could not delete audio: {why}");
            });

            sleep_until(Instant::now() + Duration::from_secs(60 * 60 * 24)).await;
        }
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageData {
    pub message_id: MessageId,
    pub message_content: String,
    pub author_id: UserId,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
}

impl MessageData {
    pub fn new(
        message_id: MessageId,
        message_content: &String,
        author_id: UserId,
        channel_id: ChannelId,
        guild_id: Option<GuildId>,
    ) -> Self {
        Self {
            message_id,
            message_content: message_content.to_owned(),
            author_id,
            channel_id,
            guild_id,
        }
    }

    pub async fn get_message_data(message_id: &MessageId) -> SurrealResult<Option<Self>> {
        DB.use_ns("discord-namespace").use_db("discord").await?;
        let sql_query = "SELECT * FROM messages WHERE message_id = $message_id";
        let existing_data: Option<Self> = DB
            .query(sql_query)
            .bind(("message_id", message_id))
            .await?
            .take(0)?;

        Ok(existing_data)
    }
}