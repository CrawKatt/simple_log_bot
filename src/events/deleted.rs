use chrono::Utc;
use poise::serenity_prelude as serenity;
use serenity::all::{ChannelId, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, GuildId, Member, Message, MessageId, Timestamp, UserId};
use crate::config::LOG_CHANNEL_ID;

use crate::database::MessageData;
use crate::events::error::CommandResult;

pub async fn handler(ctx: &serenity::Context, deleted_message_id: &MessageId) -> CommandResult {
    let message_id = deleted_message_id;

    let message_data = MessageData::get_message_data(message_id).await?;
    if message_data.is_none() { return Ok(()) }
    let message_data = message_data.ok_or("No se encontró el mensaje en la base de datos.")?;
    let guild_id = message_data.guild_id.ok_or("No se encontró el ID del servidor o se está interactuando con el Bot por DM")?;

    send_embed(ctx, guild_id, &message_data.channel_id, message_data.author_id, &message_data.message_content).await?;

    Ok(())
}

// LOS EMBEDS NO NOTIFICAN SI SE MENCIONA CON @ A UN USUARIO
pub async fn send_embed(
    ctx: &serenity::Context,
    guild_id: GuildId,
    delete_channel_id: &ChannelId,
    author_id: UserId,
    message_content: &String,
) -> serenity::Result<Message> {
    let author_user = guild_id.member(&ctx.http, author_id).await?;
    let description = format!("Autor del mensaje: <@{author_id}>\nCanal de origen: <#{delete_channel_id}>\nContenido del mensaje: {message_content}");
    let footer = "Nota: Las menciones a usuarios con @ no mencionan a los usuarios si están dentro de un embed.";
    let embed = create_embed_common(&author_user, "⚠️ Mensaje eliminado", &description, footer);
    let timestamp: Timestamp = Utc::now().into();

    LOG_CHANNEL_ID.send_message(&ctx.http, CreateMessage::default().embed(embed.timestamp(timestamp))).await
}

fn create_embed_common(author_user: &Member, title: &str, description: &str, footer: &str) -> CreateEmbed {
    CreateEmbed::default()
        .title(title)
        .description(description)
        .author(CreateEmbedAuthor::new(&author_user.distinct())
            .name(&author_user.distinct())
            .icon_url(author_user.face()))
        .color(0xFF0000)
        .footer(CreateEmbedFooter::new(footer))
}