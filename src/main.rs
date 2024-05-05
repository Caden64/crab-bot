use std::collections::HashMap;
use poise::serenity_prelude as serenity;
use std::default::Default;
use poise::serenity_prelude::FullEvent::GuildMemberAddition;
use poise::serenity_prelude::Unresolved::RoleId;
use serde::Deserialize;
use poise::serenity_prelude::model::guild::Role;

#[derive(Debug, Deserialize)]
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Deserialize)]
struct ConfigData{
    token: HashMap<String, String>,
    roles: HashMap<String, u64>,
    channels: HashMap<String, u64>,
    features: HashMap<String, bool>
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

#[poise::command(slash_command, ephemeral, hide_in_help)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
   Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn register(
    ctx: Context<'_>,
    name: String,
    email: String,
    interests: String,
    university: String,
    email_distro: Option<bool>,
) -> Result<(), Error> {
    let email_distro = email_distro.unwrap_or_default();
    ctx.reply(format!("{} {} {} {} {}", name, email, interests, university, email_distro)).await?;
   Ok(())
}

const DISCORD_TOKEN: &str = "discord_token";
const REMOVE_ROLE_ID: &str = "REMOVE_ROLE_ID";

#[tokio::main]
async fn main() {
    let config = get_config().expect("Unable to properly decode config.toml file");
    let token = config.token.get(DISCORD_TOKEN).expect("Unable to find discord_token in config.toml file");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            commands: vec![register_commands(), ping(), register()],
            event_handler: |ctx, event, framework, data | {
                Box::pin(event_handler(ctx, event, framework, data))   
            },
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

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, Data, Error>,
    _: &Data,
) -> Result<(), Error>{
    if let GuildMemberAddition { new_member, .. } = event {
        println!("{} Joined Server", new_member.user.name);
        let config = get_config().unwrap();

        let default_role_id = *config.roles.get(REMOVE_ROLE_ID).expect("UNABLE TO GET REMOVE ROLE ID");


        new_member.add_role(&ctx.http, default_role_id).await?;
    };
    Ok(())
}