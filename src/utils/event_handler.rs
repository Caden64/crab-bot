use poise::serenity_prelude as serenity;
use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Message, Ready, VoiceStateUpdate};
use poise::serenity_prelude::{CacheHttp, ChannelId, CreateMessage, Mentionable};
use regex::Regex;

use crate::utils::config::READING_CHANNEL;
use crate::utils::handle_voice_state_update::handle_voice_state_update;
use crate::{Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        // Bot successfully logged in
        Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }

        // New member joined the server
        GuildMemberAddition { new_member, .. } => {
            let member_guild = new_member.guild_id.get();
            if data.config_data.guild.main.guild_id == member_guild {
                let join_message = CreateMessage::new().content(format!(
                    "Welcome to {}, {}! Thanks for joining us! 🎉",
                    ctx.http().get_guild(new_member.guild_id).await?.name,
                    new_member.user.mention(),
                ));
                new_member
                    .user
                    .create_dm_channel(ctx.http.http())
                    .await?
                    .send_message(ctx.http.http(), join_message)
                    .await?;
            }
            println!(
                "{} Joined Server {} ",
                new_member.user.name,
                ctx.http().get_guild(new_member.guild_id).await?.name
            );
        }
        VoiceStateUpdate { new, old } => {
            handle_voice_state_update(new, old, ctx, framework, data).await;
        }
        // Fallback for other types of event
        Message { new_message } => {
            let x = framework.user_data  .config_data.features.get("SEND_NEWS");
            if let Some(y) = x {
                if !y {
                    return Ok(());
                } 
            }
            if new_message.channel_id == *data.config_data.channels.get(READING_CHANNEL).unwrap() {
                let http_match = Regex::new(r"^(https|http|\^\^).*").unwrap();
                if http_match.is_match(&new_message.content) {
                    for partner in data.config_data.guild.partners.clone() {
                        println!("FOR PARTNER {}", partner.0);
                        if partner.1.send_news {
                            let message = CreateMessage::new().content(format!(
                                "*This was originally posted by `{}`:*\n{}",
                                new_message.author.name, new_message.content
                            ));
                            ctx.http
                                .get_channel(ChannelId::new(partner.1.news_channel))
                                .await?
                                .id()
                                .send_message(ctx.http.http(), message)
                                .await?;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
