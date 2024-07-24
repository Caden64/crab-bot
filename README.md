# Crab Bot

## How to set up config file

place config.toml in the same directory as the executable
> config.toml
```toml
[TOKEN]
discord_token ="DISCORD TOKEN HERE"

[GUILD.MAIN]
GUILD_ID = 1 # the home server of the bot
PRESIDENT = 1 # who ever is in change of the bot

[GUILD.PARTNERS.a name or something]
id = 1
name = "Name of partner"
SEND_NEWS = true # whether the "partner" want's to receive the news posted or not
NEWS_CHANNEL = 1

[ROLES.PUBLIC]
# The supported roles id's that can be given to users
ROLE1 = 1
ROLE2 = 1
other = 1

[ROLES.PRIVATE]
# the default role given when a user accepts the rules
REMOVE_ROLE_ID = 1
# the role who should be notified if something goes wrong in registration (caused by user having higher permission than the bot or already existing in enrollments.json)
ADMIN_ROLE_ID = 1

[CHANNELS]
# Channel where user will enroll
ENROLL_CHANNEL_ID = 1
# A voice channel where the meetings happen
MEETING_CHANNEL_ID = 1
# where the news urls are posted to be shared 
READING_CHANNEL_ID = 1

# currently not used and can be removed
[FEATURES]
SEND_NEWS = true

```

## How to run

### Using the Rust installed locally
for development purposes you can remove the --release flag
```shell
cargo build --release
./target/release/crab-bot
```

### Docker installed locally
```shell
docker build --tag 'crab-bot-v2' .
docker run --detach 'crab-bot-v2'
```

## For future use

```rust
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use chrono_tz::America::Denver;
use clokwerk::{AsyncScheduler, TimeUnits};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CacheHttp, ChannelId, Http};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = "I work!";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
async fn one_min(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.reply("this is just so discord does not get mad!").await?;
    let now = SystemTime::now();
    let http = &ctx.serenity_context().http;
    println!("before waiting");
    // needs to say something so discord works properly
    run_one_min(Arc::clone(http)).await;
    println!("out of waiting");
    println!("time diff in seconds {}", now.elapsed().unwrap().as_secs());
    Ok(())
}

async fn run_one_min(http: Arc<Http>) -> tokio::task::JoinHandle<()>{
    println!("Going to run every ten sec");
    let mut sched = AsyncScheduler::with_tz(Denver);
    let http = Arc::clone(&http);
    sched.every(10.second()).run(move || {
        send_msg(http.clone())
    });
    tokio::spawn(async move {
        loop {
            sched.run_pending().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("wait ten seconds has been done");
        }
    })
}

async fn send_msg(cache_http: impl CacheHttp) {
    // channel bot to speak in here
    let z: ChannelId = ChannelId::new(1);
    z.say(cache_http.http(), "wow").await.expect("Should have worked");

}


#[tokio::main]
async fn main() {
    let token = "DISCORD TOKEN HERE";
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), one_min()],
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
```