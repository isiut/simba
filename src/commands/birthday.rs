// use crate::{
//     config::{DISCORD_TOKEN, MONGODB_URI},
//     Context, Error,
// };
// use mongodb::{
//     bson::{doc, Document},
//     options::{ClientOptions, FindOptions},
//     Client,
// };

// extern crate chrono;
// use chrono::{Datelike, Duration, NaiveDate, Utc};
// use futures::stream::TryStreamExt;
// use poise::serenity_prelude::{http::Http, model::prelude::UserId};
// use tokio::time::{sleep, Duration as TokioDuration};

// /// Set your birthday
// #[poise::command(slash_command)]
// pub async fn birthday(
//     ctx: Context<'_>,
//     #[description = "month"] month: i32,
//     #[description = "day"] day: i32,
// ) -> Result<(), Error> {
//     if !(day > 0 && day <= 31) {
//         ctx.say("Incorrect day value.").await?;
//         return Err(Error::from("Incorrect day value"));
//     }

//     if !(month > 0 && month <= 12) {
//         ctx.say("Incorrect month value.").await?;
//         return Err(Error::from("Incorrect month value"));
//     }

//     let current_year = Utc::now().year();
//     let mut birthday_naive = NaiveDate::from_ymd(current_year, month as u32, day as u32);

//     let _document = doc! {
//         "user": ctx.author().to_string(),
//         "next_date": "abc",
//     };

//     Ok(())
// }

// async fn add_birthday(ctx: Context<'_>, doc: Document) -> Result<(), Error> {
//     let mongodb_uri = MONGODB_URI;

//     // Connect to the database
//     let client_options = ClientOptions::parse(mongodb_uri).await.map_err(|e| {
//         eprintln!("Failed to parse MongoDB URI: {}", e);
//         Error::from(e)
//     })?;

//     // Create the client
//     let client = Client::with_options(client_options).map_err(|e| {
//         eprintln!("Failed to create MongoDB client: {}", e);
//         Error::from(e)
//     })?;

//     let db = client.database("Birthday");
//     let collection = db.collection("Birthday");

//     // Insert into the database
//     if let Err(err) = collection.insert_one(doc, None).await {
//         let response = format!("Your birthday was not set: {}", err);
//         eprintln!("{}", response);
//         ctx.say(response).await?;
//         return Err(Error::from(err));
//     }

//     Ok(())
// }

// async fn send_birthday(user_id: Option<&str>, month: Option<&str>) -> Result<(), Error> {
//     // Parse the user ID string into a UserId
//     let user_id = if let Some(mention_str) = user_id {
//         let numeric_part: String = mention_str.chars().filter(|c| c.is_ascii_digit()).collect();
//         match numeric_part.parse::<u64>() {
//             Ok(id) => UserId(id),
//             Err(err) => {
//                 println!("Invalid user ID: '{:?}'", numeric_part.parse::<u64>());
//                 return Err(Error::from(err));
//             }
//         }
//     } else {
//         println!("User ID not provided");
//         return Err(Error::from("User ID not provided"));
//     };

//     todo!();

//     Ok(())
// }

// pub async fn check_birthdays() -> Result<(), Error> {
//     loop {
//         let mongodb_uri = MONGODB_URI;

//         // Connect to the database
//         let client_options = ClientOptions::parse(mongodb_uri).await.map_err(|e| {
//             eprintln!("Failed to parse MongoDB URI: {}", e);
//             Error::from(e)
//         })?;

//         // Create the client
//         let client = Client::with_options(client_options).map_err(|e| {
//             eprintln!("Failed to create MongoDB client: {}", e);
//             Error::from(e)
//         })?;

//         let db = client.database("Remind");
//         let collection = db.collection::<Document>("Remind");

//         // Search for reminder entries
//         let filter = doc! {};
//         let find_options = FindOptions::builder().build();
//         let mut cursor = collection.find(filter, find_options).await?;

//         while let Some(reminder) = cursor.try_next().await? {
//             if let Some(date) = reminder.get("date") {
//                 if date.as_i64().unwrap() > Utc::now().timestamp() {
//                     continue;
//                 }

//                 // Delete the sent reminder from the database
//                 if let Some(user) = reminder.get("user") {
//                     if let Some(message) = reminder.get("message") {
//                         match send_reminder(user.as_str(), message.as_str()).await {
//                             Ok(_) => {
//                                 if let Some(reminder_id) = reminder.get("_id") {
//                                     let filter = doc! { "_id": reminder_id.clone() };
//                                     match collection.delete_one(filter, None).await {
//                                         Ok(_) => println!("Reminder deleted successfully"),
//                                         Err(e) => eprintln!("Failed to delete reminder: {}", e),
//                                     }
//                                 }
//                             }
//                             Err(e) => println!("{}", e),
//                         }
//                     } else {
//                         println!("Incorrect message value")
//                     }
//                 } else {
//                     println!("Incorrect user value!")
//                 }
//             } else {
//                 println!("Incorrect date value!");
//             }
//         }

//         sleep(TokioDuration::from_secs(86400)).await;
//     }
// }
