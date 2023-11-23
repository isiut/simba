use std::time::Duration;

use crate::{
    config::{BIRTHDAY_CHANNEL, DISCORD_TOKEN, MONGODB_URI},
    Context, Error,
};
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions},
    Client,
};

extern crate chrono;
use chrono::{Datelike, NaiveDate, Utc};
use futures::stream::TryStreamExt;
use poise::serenity_prelude::{ChannelId, Http};
use tokio::time::sleep;

/// Set your birthday
#[poise::command(slash_command)]
pub async fn birthday(
    ctx: Context<'_>,
    #[description = "month"] month: i32,
    #[description = "day"] day: i32,
) -> Result<(), Error> {
    if !(day > 0 && day <= 31) {
        ctx.say("Incorrect day value.").await?;
        return Err(Error::from("Incorrect day value"));
    }

    if !(month > 0 && month <= 12) {
        ctx.say("Incorrect month value.").await?;
        return Err(Error::from("Incorrect month value"));
    }

    let current_year = Utc::now().year();
    let mut birthday_naive = NaiveDate::from_ymd_opt(current_year, month as u32, day as u32)
        .expect("birthday date error");
    if birthday_naive < Utc::now().date_naive() {
        birthday_naive = NaiveDate::from_ymd_opt(current_year + 1, month as u32, day as u32)
            .expect("birthday date error");
    }
    let next_birthday = birthday_naive
        .and_hms_opt(0, 0, 0)
        .expect("birthday date error")
        .timestamp();

    let doc = doc! {
        "user": ctx.author().to_string(),
        "next_date": next_birthday,
    };

    add_birthday(ctx, doc).await?;

    Ok(())
}

async fn add_birthday(ctx: Context<'_>, doc: Document) -> Result<(), Error> {
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

    let db = client.database("Birthday");
    let collection = db.collection("Birthday");

    // Insert into the database
    if let Err(err) = collection.insert_one(doc, None).await {
        let response = format!("Your birthday was not set: {}", err);
        eprintln!("{}", response);
        ctx.say(response).await?;
        return Err(Error::from(err));
    }

    ctx.say("Birthday added successfully.").await?;

    Ok(())
}

async fn send_birthday(user_id: Option<&str>) -> Result<(), Error> {
    let message = format!("Happy birthday {}", user_id.unwrap_or("error"));
    let http = Http::new(DISCORD_TOKEN);
    ChannelId(BIRTHDAY_CHANNEL).say(http, message).await?;

    Ok(())
}

pub async fn check_birthdays() -> Result<(), Error> {
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

        let db = client.database("Birthday");
        let collection = db.collection::<Document>("Birthday");

        // Search for birthday entries
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor = collection.find(filter, find_options).await?;

        while let Some(birthday) = cursor.try_next().await? {
            if let Some(date) = birthday.get("next_date") {
                if date.as_i64().unwrap() > Utc::now().timestamp() {
                    continue;
                }

                // Send birthday and set it to one year later
                if let Some(user) = birthday.get("user") {
                    if let Some(date) = birthday.get("next_date") {
                        match send_birthday(user.as_str()).await {
                            Ok(_) => {
                                if let Some(birthday_id) = birthday.get("_id") {
                                    let filter = doc! { "_id": birthday_id.clone() };
                                    match collection.delete_one(filter, None).await {
                                        Ok(_) => {
                                            println!("birthday deleted successfully");

                                            let seconds_in_year = 31_536_000;
                                            let doc = doc! {
                                                "user": user,
                                                "next_date": date.as_i64().unwrap() + seconds_in_year as i64
                                            };

                                            if let Err(err) = collection.insert_one(doc, None).await
                                            {
                                                eprintln!(
                                                    "Error adding one year to past birthday: {}",
                                                    err
                                                );
                                                return Err(Error::from(err));
                                            }
                                        }
                                        Err(e) => eprintln!("Failed to delete birthday: {}", e),
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

        sleep(Duration::from_secs(86400)).await;
    }
}
