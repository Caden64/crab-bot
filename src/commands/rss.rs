use rss::Channel;
use std::error::Error as E2;
use crate::{Context, Error};

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn rss(ctx: Context<'_>, 
                 #[min = 1]
                 #[max = 10]
                 n: u8,
) -> Result<(), Error> {
    let data = &ctx.data().config_data.rss.feed;
    for i in data {
        ctx.say(format!("getting data from {}", i.0)).await?;

        if let Ok(channel) = specific_feed(&i.1.url).await {
            for i in channel.items.iter().take(n as usize) {
                if let Some(link) = &i.link {
                    if let Some(title) = &i.title {
                        ctx.say(format!("{} has a title of {}", link, title)).await?;
                    }
                }
            }
        }
    }
    ctx.say("done").await?;
    Ok(())
}

async fn specific_feed(url: &String) -> Result<Channel, Box<dyn E2 + Send>> {
    let response = match reqwest::get(url).await {
        Ok(res) => res,
        Err(e) => return Err(Box::new(e)),
    };

    let content = match response.bytes().await {
        Ok(b) => b,
        Err(e) => return Err(Box::new(e)),
    };

    let channel = match Channel::read_from(&content[..]) {
        Ok(ch) => ch,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(channel)
}