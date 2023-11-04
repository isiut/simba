use crate::{config::OPENWEATHER_API_KEY, Context, Error};
extern crate openweathermap;
use openweathermap::weather as weather_api;

#[derive(poise::ChoiceParameter)]
pub enum UnitChoices {
    #[name = "metric"]
    Metric,
    #[name = "imperial"]
    Imperial,
}

#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Location"] location: String,
    #[description = "Units"] units: UnitChoices,
) -> Result<(), Error> {
    let result = match &weather_api(
        &location,
        match units {
            UnitChoices::Metric => "metric",
            UnitChoices::Imperial => "imperial",
        },
        "en",
        OPENWEATHER_API_KEY,
    )
    .await
    {
        Ok(current) => {
            let temp_unit = match units {
                UnitChoices::Metric => "C",
                UnitChoices::Imperial => "F",
            };

            format!(
                "Weather for: {}, {} ({}, {})
                Conditions: {}
                Temperature: {} °{} (Feels like {} °{})",
                current.name.as_str(),
                current.sys.country,
                current.coord.lat,
                current.coord.lon,
                current.weather[0].main,
                current.main.temp,
                temp_unit,
                current.main.feels_like,
                temp_unit
            )
        }
        Err(e) => format!("Could not fetch weather because: {}", e),
    };

    ctx.say(result).await?;

    Ok(())
}
