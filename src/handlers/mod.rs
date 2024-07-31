pub mod anti_spam;

use poise::serenity_prelude as serenity;
use crate::{Data, events};
use crate::events::error::{CommandResult, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>
) -> CommandResult {
    match event {
        serenity::FullEvent::Message { new_message } => events::messages::handler(ctx, new_message).await?,
        serenity::FullEvent::MessageUpdate { event, .. } => events::updated::handler(ctx, event).await?,
        serenity::FullEvent::MessageDelete { deleted_message_id, .. } => events::deleted::handler(ctx, deleted_message_id).await?,
        _ => println!("Event: {:?}", event.snake_case_name())
    }

    Ok(())
}