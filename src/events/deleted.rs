use serenity::all::{ChannelId, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, Message, MessageId, User, UserId};
use crate::events::error::CommandResult;
use poise::serenity_prelude as serenity;
use crate::database::MessageData;

pub async fn handler(ctx: &serenity::Context, _channel_id: &ChannelId, deleted_message_id: &MessageId) -> CommandResult {
    let message_id = deleted_message_id;

    let message_data = MessageData::get_message_data(message_id).await?;
    if message_data.is_none() { return Ok(()) }
    let message_data = message_data.ok_or("No se encontró el mensaje en la base de datos.")?;
    let log_channel_id = ChannelId::new(1230367709058039828);

    send_embed(ctx, log_channel_id, &message_data.channel_id, message_data.author_id, &message_data.message_content).await?;

    Ok(())
}

// LOS EMBEDS NO NOTIFICAN SI SE MENCIONA CON @ A UN USUARIO
pub async fn send_embed(
    ctx: &serenity::Context,
    log_channel_id: ChannelId,
    delete_channel_id: &ChannelId,
    author_id: UserId,
    message_content: &String,
) -> serenity::Result<Message> {
    let author_user = author_id.to_user(&ctx.http).await?;
    let description = format!("Autor del mensaje: <@{author_id}>\nCanal de origen: <#{delete_channel_id}>\nContenido del mensaje: {message_content}");
    let footer = "Nota: Las menciones a usuarios con @ no mencionan a los usuarios si están dentro de un embed.";
    let embed = create_embed_common(&author_user, "Mensaje eliminado", &description, footer);

    log_channel_id.send_message(&ctx.http, CreateMessage::default().embed(embed)).await
}

fn create_embed_common(author_user: &User, title: &str, description: &str, footer: &str) -> CreateEmbed {
    CreateEmbed::default()
        .title(title)
        .description(description)
        .author(CreateEmbedAuthor::new(&author_user.name)
            .name(&author_user.name)
            .icon_url(author_user.avatar_url().unwrap_or_else(|| author_user.default_avatar_url())))
        .color(0x0000_ff00)
        .footer(CreateEmbedFooter::new(footer))
}