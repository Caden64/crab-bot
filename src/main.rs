mod checks;
mod commands;
mod storage;
mod utils;
use crate::commands::acknowledgement::acknowledge;
use crate::commands::enroll::enroll;
use crate::commands::help::help;
use crate::commands::modal::admin_create_modal;
use crate::commands::news::news;
use crate::commands::ping::ping;
use crate::commands::register_commands::register_commands;
use crate::utils::config::{get_config, ConfigData, DISCORD_TOKEN, REMOVE_ROLE_ID};
use crate::utils::event_handler::event_handler;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GatewayIntents;
use serde::Deserialize;
use std::default::Default;
use std::sync::atomic::AtomicBool;

// define types for bot use
#[derive(Debug, Deserialize)]
struct Data {
    config_data: ConfigData,
    _meeting_time: AtomicBool,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load configuration
    let config = get_config().expect("Unable to properly decode config.toml file");
    // Retrieve Discord token from configuration
    let token = config
        .token
        .get(DISCORD_TOKEN)
        .expect("Unable to find discord_token in config.toml file")
        .to_string();

    // intents
    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    // Build the bot framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            pre_command: |ctx| {
                Box::pin(async move {
                    // Log the invoked command and the author
                    println!(
                        "{} called: {}",
                        ctx.author().name,
                        ctx.invoked_command_name(),
                    );
                })
            },
            on_error: |error| Box::pin(on_error(error)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            // Register bot commands
            // for now only have the basic commands for enrollment
            commands: vec![
                ping(),
                enroll(),
                help(),
                acknowledge(),
                news(),
                admin_create_modal(),
                #[cfg(debug_assertions)]
                register_commands(),
            ],
            // Handler for other events
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Globally register commands
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config_data: config.clone(),
                    _meeting_time: AtomicBool::new(false),
                })
            })
        })
        .build();

    // Create a bot client with the token, intents and framework
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    // Start the bot client
    client.unwrap().start().await.unwrap();
}
