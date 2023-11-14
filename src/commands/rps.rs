use crate::{Context, Error};
use rand::Rng;
use std::collections::HashMap;

#[derive(poise::ChoiceParameter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponChoices {
    #[name = "rock"]
    Rock,
    #[name = "paper"]
    Paper,
    #[name = "scissors"]
    Scissors,
}

/// Play Rock-Paper-Scissors
#[poise::command(slash_command)]
pub async fn rps(
    ctx: Context<'_>,
    #[description = "Your choice"] user_choice: WeaponChoices,
) -> Result<(), Error> {
    let weapons = vec![
        WeaponChoices::Rock,
        WeaponChoices::Paper,
        WeaponChoices::Scissors,
    ];
    let bot_choice = weapons[rand::thread_rng().gen_range(0..2)];

    // Key beats value
    let beats = HashMap::from([
        (WeaponChoices::Rock, WeaponChoices::Scissors),
        (WeaponChoices::Paper, WeaponChoices::Rock),
        (WeaponChoices::Scissors, WeaponChoices::Paper),
    ]);

    let response = match (user_choice, bot_choice) {
        (user_choice, bot_choice) if user_choice == bot_choice => {
            format!(
                "You chose {} and it's a draw!\nThe bot chose {:?}.",
                user_choice, bot_choice
            )
        }
        (user_choice, bot_choice) if beats.get(&user_choice) == Some(&bot_choice) => {
            format!(
                "You chose {} and you won!\nThe bot chose {:?}.",
                user_choice, bot_choice
            )
        }
        (user_choice, bot_choice) if beats.get(&bot_choice) == Some(&user_choice) => {
            format!(
                "You chose {} lost!\nThe bot chose {:?}.",
                user_choice, bot_choice
            )
        }
        _ => format!("An unexpected error occurred."),
    };

    ctx.say(response).await?;
    Ok(())
}
