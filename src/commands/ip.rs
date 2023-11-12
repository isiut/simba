use crate::{Context, Error};

/// Get the server's IP
#[poise::command(slash_command)]
pub async fn ip(ctx: Context<'_>) -> Result<(), Error> {
    let response = "64.227.172.38".to_string();
    ctx.say(response).await?;
    Ok(())
}
