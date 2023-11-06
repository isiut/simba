use crate::{Context, Error};

/// Get the server's IP
#[poise::command(slash_command)]
pub async fn ip(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("64.227.172.38");
    ctx.say(response).await?;
    Ok(())
}
