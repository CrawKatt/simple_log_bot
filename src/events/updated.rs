use chrono::Utc;
use poise::serenity_prelude as serenity;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage, MessageUpdateEvent, Timestamp};
use serenity::builder::CreateEmbedAuthor;

use crate::config::LOG_CHANNEL_ID;
use crate::database::MessageData;
use crate::events::error::CommandResult;

pub async fn handler(ctx: &serenity::Context, event: &MessageUpdateEvent) -> CommandResult {
    if event.author.as_ref().map_or(false, |author| author.bot) { return Ok(()) }

    let guild_id = event.guild_id.ok_or("No se encontró el ID del servidor o se está interactuando con el Bot por DM")?;
    let message_id = event.id;
    let old_message = MessageData::get_message_data(&message_id).await?;
    let Some(database_message) = old_message else { return Ok(()) };

    let author_id = database_message.author_id;
    let author = guild_id.member(ctx, author_id).await?;
    let timestamp: Timestamp = Utc::now().into();
    let old_content = &database_message.message_content;
    let new_content = event.content.as_deref().ok_or("No se enconró un mensaje")?;
    if old_content == new_content { return Ok(()) }
    let channel_id = event.channel_id;
    let embed = CreateEmbed::default()
        .title("⚠️ Mensaje Editado")
        .author(CreateEmbedAuthor::new(author.distinct())
            .name(author.distinct())
            .icon_url(author.user.face()))
        .description(format!("**Autor del mensaje**: <@{author_id}>\n**Canal**: <#{channel_id}>\n**Mensaje Antiguo**: {old_content}\n**Mensaje Nuevo**: {new_content}"))
        .footer(CreateEmbedFooter::new("Nota: Las menciones a usuarios con @ no mencionan a los usuarios si están dentro de un embed."))
        .color(0xFF0000);
    let builder = CreateMessage::new().embed(embed.timestamp(timestamp));
    LOG_CHANNEL_ID.send_message(&ctx.http, builder).await?;

    Ok(())
}