use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("Ping");
    ctx.say(response).await?;
    Ok(())
}
