mod commands;
mod gameslist;

use poise::serenity_prelude as serenity;

pub struct Data {}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' missing");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping(), commands::gameslist_command()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
