mod commands;
mod utils;
mod checks;

use poise::serenity_prelude as serenity;
use std::default::Default;
use serde::Deserialize;
use crate::commands::help::help;
use crate::commands::ping::ping;
use crate::commands::register::register;
use crate::commands::register_commands::register_commands;
use crate::commands::modal::component_modal;
use crate::utils::config::{DISCORD_TOKEN, REMOVE_ROLE_ID, GUILD_ID, get_config, ConfigData};
use crate::utils::event_handler::event_handler;

#[derive(Debug, Deserialize)]
struct Data {
    config_data: ConfigData
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let config = get_config().expect("Unable to properly decode config.toml file");
    let token = config.token.get(DISCORD_TOKEN).expect("Unable to find discord_token in config.toml file").to_string();
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            commands: vec![ping(), register(), register_commands(), help(), component_modal()],
            event_handler: |ctx, event, framework, data | {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {config_data: config.clone()})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
