use crate::utils::config::Feed;
use crate::{Context, Error};
use chrono::DateTime;
use chrono_tz::America::Denver;
use clokwerk::{AsyncScheduler, TimeUnits};
use poise::serenity_prelude::{CacheHttp, Channel, ChannelId, Http};
use rss::Channel as rssChannel;
use std::collections::HashMap;
use std::error::Error as E2;
use std::sync::Arc;
use std::time::Duration;

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn rss(ctx: Context<'_>, channel: Channel) -> Result<(), Error> {
    let http = &ctx.serenity_context().http;
    run_one_hour(
        Arc::clone(http),
        &ctx.data().config_data.rss.feed,
        channel.id().get(),
    )
    .await;
    ctx.say(format!("done {}", channel.id().get())).await?;
    Ok(())
}

async fn run_one_hour(
    http: Arc<Http>,
    urls: &HashMap<String, Feed>,
    channel_id: u64,
) -> tokio::task::JoinHandle<()> {
    println!("Going to run every hour");
    let mut sched = AsyncScheduler::with_tz(Denver);
    let http = Arc::clone(&http);
    let feed_urls = urls
        .values()
        .map(|x| x.url.clone())
        .collect::<Vec<String>>();
    sched
        .every(1.hour())
        .run(move || send_news(http.clone(), channel_id, feed_urls.clone()));
    tokio::spawn(async move {
        loop {
            sched.run_pending().await;
            tokio::time::sleep(Duration::from_secs(3605)).await;
            println!("I have have been falling! - Loki");
        }
    })
}

async fn send_news(cache_http: impl CacheHttp, channel: u64, feed_urls: Vec<String>) {
    let discord_channel: ChannelId = ChannelId::new(channel);
    for feed in feed_urls {
        if let Ok(rss_channel) = specific_feed(&feed).await {
            for item in rss_channel.items {
                if good_date(item.pub_date).await {
                    if let Some(link) = item.link {
                        if let Some(title) = item.title {
                            let send_message = discord_channel
                                .say(
                                    cache_http.http(),
                                    format!("Article: title: {}, link: {}", title, link),
                                )
                                .await;
                            if let Err(message_error) = send_message {
                                println!("RAN INTO ERROR: {}", message_error)
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn good_date(date: Option<String>) -> bool {
    if let Some(rfc2822_date) = date {
        if let Ok(good_date) = parse_and_compare_date(rfc2822_date.as_str()).await {
            if good_date {
                return true;
            }
        }
    }
    false
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
async fn specific_feed(url: &String) -> Result<rssChannel, Box<dyn E2 + Send>> {
    let response = match reqwest::get(url).await {
        Ok(res) => res,
        Err(e) => return Err(Box::new(e)),
    };

    let content = match response.bytes().await {
        Ok(b) => b,
        Err(e) => return Err(Box::new(e)),
    };

    let channel = match rssChannel::read_from(&content[..]) {
        Ok(ch) => ch,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(channel)
}
