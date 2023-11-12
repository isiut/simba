use crate::{
    config::{DISCORD_TOKEN, MONGODB_URI},
    Context, Error,
};
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions},
    Client,
};
extern crate chrono;
use chrono::{Duration, Utc};
use futures::stream::TryStreamExt;
use poise::serenity_prelude::{http::Http, model::prelude::UserId};
use tokio::time::{sleep, Duration as TokioDuration};

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

            // Connect to the database
            let client_options = ClientOptions::parse(mongodb_uri).await.map_err(|e| {
                eprintln!("Failed to parse MongoDB URI: {}", e);
                Error::from(e)
            })?;

            // Create the client
            let client = Client::with_options(client_options).map_err(|e| {
                eprintln!("Failed to create MongoDB client: {}", e);
                Error::from(e)
            })?;

            let db = client.database("Remind");
            let collection = db.collection("Remind");

            // Create the database entry
            let caller = ctx.author();
            let remind_date = (Utc::now() + time).timestamp();
            let document = doc! {
                "user": caller.to_string(),
                "message": &message,
                "date": remind_date,
            };

            // Insert into the database
            if let Err(err) = collection.insert_one(document, None).await {
                eprintln!("Failed to insert document into MongoDB: {}", err);
                return Err(Error::from(err));
            }

            // Success message
            let response = format!("You will be reminded of '{}' in {}", message, wait_for);
            ctx.say(&response).await?;

            let dm = caller.create_dm_channel(&ctx).await?;
            dm.say(&ctx, response).await?;
        }
        None => {
            // Failure message
            let response = "There was an error adding your reminder. Please check that you set the time correctly. The valid durations are s, m, h, d, w.".to_string();
            ctx.say(response).await?;
        }
    }

    Ok(())
}

async fn send_reminder(user_id: Option<&str>, message: Option<&str>) -> Result<(), Error> {
    // Parse the user ID string into a UserId
    let user_id = if let Some(mention_str) = user_id {
        let numeric_part: String = mention_str.chars().filter(|c| c.is_ascii_digit()).collect();
        match numeric_part.parse::<u64>() {
            Ok(id) => UserId(id),
            Err(err) => {
                println!("Invalid user ID: '{:?}'", numeric_part.parse::<u64>());
                return Err(Error::from(err));
            }
        }
    } else {
        println!("User ID not provided");
        return Err(Error::from("User ID not provided"));
    };

    // Create the user
    let http = Http::new(DISCORD_TOKEN);
    let user = match user_id.to_user(&http).await {
        Ok(user) => user,
        Err(_) => {
            println!("User not found");
            return Err(Error::from("User not found"));
        }
    };

    // DM the reminder
    if let Some(message) = message {
        let dm = user.create_dm_channel(&http).await?;
        dm.say(http, message).await?;
    }

    Ok(())
}

pub async fn check_reminders() -> Result<(), Error> {
    loop {
        let mongodb_uri = MONGODB_URI;

        // Connect to the database
        let client_options = ClientOptions::parse(mongodb_uri).await.map_err(|e| {
            eprintln!("Failed to parse MongoDB URI: {}", e);
            Error::from(e)
        })?;

        // Create the client
        let client = Client::with_options(client_options).map_err(|e| {
            eprintln!("Failed to create MongoDB client: {}", e);
            Error::from(e)
        })?;

        let db = client.database("Remind");
        let collection = db.collection::<Document>("Remind");

        // Search for reminder entries
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor = collection.find(filter, find_options).await?;

        while let Some(reminder) = cursor.try_next().await? {
            if let Some(date) = reminder.get("date") {
                if date.as_i64().unwrap() > Utc::now().timestamp() {
                    continue;
                }

                // Delete the sent reminder from the database
                if let Some(user) = reminder.get("user") {
                    if let Some(message) = reminder.get("message") {
                        match send_reminder(user.as_str(), message.as_str()).await {
                            Ok(_) => {
                                if let Some(reminder_id) = reminder.get("_id") {
                                    let filter = doc! { "_id": reminder_id.clone() };
                                    match collection.delete_one(filter, None).await {
                                        Ok(_) => println!("Reminder deleted successfully"),
                                        Err(e) => eprintln!("Failed to delete reminder: {}", e),
                                    }
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    } else {
                        println!("Incorrect message value")
                    }
                } else {
                    println!("Incorrect user value!")
                }
            } else {
                println!("Incorrect date value!");
            }
        }

        sleep(TokioDuration::from_secs(30)).await;
    }
}
