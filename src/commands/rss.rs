use crate::{Context, Error};
use chrono::DateTime;
use chrono_tz::America::Denver;
use rss::Channel;
use std::error::Error as E2;

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn rss(ctx: Context<'_>) -> Result<(), Error> {
    let data = &ctx.data().config_data.rss.feed;
    for i in data {
        ctx.say(format!("getting data from {}", i.0)).await?;
        if let Ok(channel) = specific_feed(&i.1.url).await {
            for i in channel.items {
                if let Some(rfc2822_date) = i.pub_date {
                    process_item(&ctx, &rfc2822_date, &i.link, &i.title).await?;
                }
            }
        }
    }
    ctx.say("done").await?;
    Ok(())
}

async fn process_item(
    ctx: &Context<'_>,
    rfc2822_date: &str,
    link: &Option<String>,
    title: &Option<String>,
) -> Result<(), Error> {
    if let Ok(good_date) = parse_and_compare_date(rfc2822_date).await {
        if good_date {
            if let Some(link) = link {
                if let Some(title) = title {
                    ctx.say(format!("{} has a title of {}", link, title))
                        .await?;
                }
            }
        }
    }
    Ok(())
}

async fn parse_and_compare_date(rfc2822_date: &str) -> Result<bool, Box<dyn E2 + Send>> {
    let pre_date = match DateTime::parse_from_rfc2822(rfc2822_date) {
        Ok(date) => date,
        Err(e) => return Err(Box::new(e)),
    };
    let local_date = pre_date.naive_local().and_local_timezone(Denver);
    if let Some(local_date) = local_date.latest() {
        if let Some(one_hour_ago) = chrono::Utc::now()
            .with_timezone(&Denver)
            .checked_sub_signed(chrono::Duration::hours(1))
        {
            return Ok(local_date > one_hour_ago);
        }
    }
    Ok(false)
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
