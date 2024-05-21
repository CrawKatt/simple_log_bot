use poise::serenity_prelude as serenity;
use serenity::all::{ChannelId, MessageUpdateEvent};

pub async fn handler(ctx: Serenity::Context, event: &MessageUpdateEvent) -> CommandResult {
    if event.author.as_ref().map_or(false, |author| author.bot) {
        return Ok(());
    }

    let message_id = event.id;
    let guild_id = event.guild_id.unwrap_or_default(); // SAFETY: El GuildId siempre está disponible
    let database_message = MessageData::get_message_data(message_id);
    let Some(database_message) = old_message else {
        return Ok(());
    };

    let old_content = &database_message.message_content;
    let new_content = event.content.as_deref().into_result()?;
    if old_content == new_content {
        return Ok(());
    }

    Ok(())
}
