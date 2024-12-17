use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, guild_only, ephemeral)]
async fn index(ctx: Context<'_>) -> Result<(), Error>{
    reqwest::get("http://127.0.0.1:8080").await.unwrap();
    ctx.say("got it").await?;
    Ok(())
}

#[poise::command(slash_command, guild_only, ephemeral)]
async fn size(ctx: Context<'_>) -> Result<(), Error>{
    let data = reqwest::get("http://127.0.0.1:8080/stats").await.unwrap();
    let body = data.text().await?;
    ctx.say(format!("index gotten: {}", body)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = "DISCORD_TOKEN".to_string();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![index(), size()],
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