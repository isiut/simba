use crate::{Context, Error};
extern crate rand;
use rand::prelude::*;

#[poise::command(slash_command)]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let choice: f64 = rand::thread_rng().gen::<f64>().round();
    let selector = choice as usize;
    let possibilities = ["Heads", "Tails"];

    let response = format!("It's {}!", possibilities[selector]);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn diceroll(
    ctx: Context<'_>,
    #[description = "Number of sides"] sides: i32,
) -> Result<(), Error> {
    let result = match sides {
        sides if sides > 1 => {
            let choice = rand::thread_rng().gen_range(1..sides);
            format!("{}", choice.to_string())
        }
        _ => format!("Enter a number that is greater than 1"),
    };
    ctx.say(result).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn rng(
    ctx: Context<'_>,
    #[description = "min"] min: i32,
    #[description = "max"] max: i32,
) -> Result<(), Error> {
    let result = match (min, max) {
        _correct_values if min < max => {
            let choice = rand::thread_rng().gen_range(min..max);
            format!("{}", choice.to_string())
        }
        _ => format!("Min should be less than max"),
    };
    ctx.say(result).await?;
    Ok(())
}
