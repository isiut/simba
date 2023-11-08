use poise::serenity_prelude as serenity;

pub mod config;
use config::DISCORD_TOKEN;

pub mod commands;
use commands::{
    ip::ip, ping::ping, poll::poll, random::coinflip, random::diceroll, random::rng,
    remind::remind, weather::timezone, weather::weather,
};

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                poll(),
                weather(),
                ip(),
                timezone(),
                coinflip(),
                diceroll(),
                rng(),
                remind(),
            ],
            ..Default::default()
        })
        .token(DISCORD_TOKEN)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
