use std::collections::HashMap;
use poise::serenity_prelude as serenity;
use std::default::Default;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Deserialize)]
struct ConfigData{
    token: HashMap<String, String>
}

fn get_config() -> Result<ConfigData, toml::de::Error>  {
    let data = std::fs::read_to_string("config.toml").expect("Unable to find config.toml file");
    toml::from_str(&data)
}

#[poise::command(prefix_command, hide_in_help, guild_only, ephemeral, slash_command, owners_only)]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
   Ok(()) 
}

const DISCORD_TOKEN: &str = "discord_token";

#[tokio::main]
async fn main() {
    let config = get_config().expect("Unable to properly decode config.toml file");
    let token = config.token.get(DISCORD_TOKEN).expect("Unable to find discord_token in config.toml file");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            commands: vec![register_commands(), ping()], 
            
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
