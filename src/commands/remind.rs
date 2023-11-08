use crate::{config::MONGODB_URI, Context, Error};
use mongodb::{bson::doc, options::ClientOptions, Client};
extern crate chrono;
use chrono::{Duration, Utc};

fn parse_duration_string(duration_string: &str) -> Option<Duration> {
    let (numeric, unit): (String, String) = duration_string.chars().partition(|c| c.is_numeric());

    if let Ok(value) = numeric.parse::<i64>() {
        Some(match unit.as_str() {
            "s" => Duration::seconds(value),
            "m" => Duration::minutes(value),
            "h" => Duration::hours(value),
            "d" => Duration::days(value),
            "w" => Duration::weeks(value),
            _ => return None,
        })
    } else {
        None
    }
}

/// Get reminded about something
#[poise::command(slash_command)]
pub async fn remind(
    ctx: Context<'_>,
    #[description = "Reminder message"] message: String,
    #[description = "Remind in (time)"] wait_for: String,
) -> Result<(), Error> {
    let wait_for_time = parse_duration_string(&wait_for);
    match wait_for_time {
        Some(time) => {
            let mongodb_uri = MONGODB_URI;

            let client_options = ClientOptions::parse(mongodb_uri).await.map_err(|e| {
                eprintln!("Failed to parse MongoDB URI: {}", e);
                Error::from(e)
            })?;

            let client = Client::with_options(client_options).map_err(|e| {
                eprintln!("Failed to create MongoDB client: {}", e);
                Error::from(e)
            })?;

            let db = client.database("Remind");
            let collection = db.collection("Remind");

            let remind_date = (Utc::now() + time).timestamp();

            let document = doc! {
                "message": message,
                "date": remind_date,
            };

            if let Err(err) = collection.insert_one(document, None).await {
                eprintln!("Failed to insert document into MongoDB: {}", err);
                return Err(Error::from(err));
            }

            let response = format!("Your reminder was added to the database.");
            ctx.say(response).await?;
        }
        None => {
            let response = format!(
                "There was an error adding your reminder. Please check that you set the time correctly. The valid durations are s, m, h, d, w."
            );
            ctx.say(response).await?;
        }
    }

    Ok(())
}
