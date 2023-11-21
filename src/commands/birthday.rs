use crate::{config::MONGODB_URI, Context, Error};
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

#[derive(poise::ChoiceParameter)]
pub enum Months {
    #[name = "January"]
    January,
    #[name = "February"]
    February,
    #[name = "March"]
    March,
    #[name = "April"]
    April,
    #[name = "May"]
    May,
    #[name = "June"]
    June,
    #[name = "July"]
    July,
    #[name = "August"]
    August,
    #[name = "September"]
    September,
    #[name = "October"]
    October,
    #[name = "November"]
    November,
    #[name = "December"]
    December,
}

/// Set your birthday
#[poise::command(slash_command)]
pub async fn birthday(
    ctx: Context<'_>,
    #[description = "month"] month: Months,
    #[description = "day"] day: i32,
) -> Result<(), Error> {
    match day {
        day if (day < 31 && day > 0) => {
            let response = format!("Setting your birthday as {} {}", month, day);
            ctx.say(response).await?;

            let document = doc! {
                "user": ctx.author().to_string(),
                "month": month.to_string(),
                "day": day
            };
            add_birthday(ctx, document).await?;
        }
        _ => {
            ctx.say("Invalid day").await?;
        }
    }

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

    Ok(())
}
