use crate::{Context, Error};
use poise::serenity_prelude::ReactionType;

/// Create a poll
#[poise::command(slash_command)]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "Poll prompt"] prompt: String,
) -> Result<(), Error> {
    // Send a response and store it
    let respond = ctx.say(prompt).await?;
    let response = respond.into_message().await?;

    // React to the response message
    let positive = ReactionType::Unicode("✅".to_string());
    let negative = ReactionType::Unicode("❌".to_string());
    response.react(ctx, positive).await?;
    response.react(ctx, negative).await?;

    Ok(())
}
