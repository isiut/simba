use crate::{Context, Error};

/// Check if the bot is online
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Pong".to_string();
    ctx.say(response).await?;
    Ok(())
}
