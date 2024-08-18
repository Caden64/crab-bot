use rss::Channel;
use std::error::Error as E2;
use chrono::DateTime;
use chrono_tz::America::Denver;
use crate::{Context, Error};

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn rss(ctx: Context<'_>, 
) -> Result<(), Error> {
    let data = &ctx.data().config_data.rss.feed;
    for i in data {
        ctx.say(format!("getting data from {}", i.0)).await?;

        if let Ok(channel) = specific_feed(&i.1.url).await {
            for i in channel.items{
                if let Some(rfc2822_date) = i.pub_date  {
                    let pre_date = DateTime::parse_from_rfc2822(&rfc2822_date);
                    if let Ok(date) = pre_date {
                        let local_date = date.naive_local().and_local_timezone(Denver);
                        if let Some(y) = local_date.latest() {
                            if y > chrono::Utc::now().with_timezone(&Denver).checked_sub_signed(chrono::Duration::hours(1)).unwrap_or_else(|| chrono::Utc::now().with_timezone(&Denver)) {
                                if let Some(link) = i.link {
                                    if let Some(title) = i.title {
                                        ctx.say(format!("{} has a title of {}", link, title)).await?;
                                    }
                                }
                            }
                        }
                        
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