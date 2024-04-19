use std::sync::Arc;
use serenity::all::{ChannelId, GuildId, Message, MessageId, UserId};
use poise::serenity_prelude as serenity;
use crate::database::{DB, MessageData};
use crate::events::error::CommandResult;

pub async fn handler(_ctx: &serenity::Context, new_message: &Message) -> CommandResult {
    DB.use_ns("discord-namespace").use_db("discord").await?;
    if new_message.author.bot { return Ok(()) }

    let guild_id = new_message.guild_id;
    let message_content = Arc::new(&new_message.content);
    let author_id = new_message.author.id;
    let channel_id = new_message.channel_id;
    let message_data = create_message_data(new_message.id, *message_content, author_id, channel_id, guild_id);
    let _created: Vec<MessageData> = DB
        .create("messages")
        .content(message_data)
        .await?;

    Ok(())
}

fn create_message_data(
    message_id: MessageId,
    message_content: &String,
    author_id: UserId,
    channel_id: ChannelId,
    guild_id: Option<GuildId>
) -> MessageData {
    MessageData::new(
        message_id,
        message_content,
        author_id,
        channel_id,
        guild_id,
    )
}