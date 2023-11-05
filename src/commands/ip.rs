use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ip(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("64.227.172.38");
    ctx.say(response).await?;
    Ok(())
}
