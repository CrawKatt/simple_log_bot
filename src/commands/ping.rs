use crate::events::error::CommandResult;
use crate::events::error::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn hello(ctx: Context<'_>) -> CommandResult {
    ctx.say("world!").await?;
    Ok(())
}