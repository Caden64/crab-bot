use std::sync::atomic::Ordering;
use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Ready, VoiceStateUpdate};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CacheHttp;
use crate::{Data, Error};
use crate::utils::config::{MEETING_CHANNEL, PRESIDENT};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data:  &Data,
) -> Result<(), Error>{
    match event {
        Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        },
        GuildMemberAddition { new_member, .. } => {
            println!("{} Joined Server", new_member.user.name);
        },
        VoiceStateUpdate { new, old } => {
            if let Some(new_member) = new.member.as_ref() {
                if old.is_none() {
                    if let Some(new_channel_id) = new.channel_id.as_ref() {
                        if let (Some(meeting_channel), Some(president)) =
                            (
                                framework.user_data.config_data.channels.get(MEETING_CHANNEL),
                                framework.user_data.config_data.guild.get(PRESIDENT)
                            ) {
                            if new_channel_id.get() == *meeting_channel
                                && new_member.user.id.get() == *president {
                                let mem_display_name = new_member.display_name();
                                if let Ok(channels) =
                                    new.guild_id.unwrap().channels(ctx.http.http()).await {
                                    if let Some(new_channel) = channels.get(new_channel_id) {
                                        if let Ok(members) =
                                            new_channel.members(ctx.cache.clone().as_ref()) {
                                            for member in members {
                                                if member.display_name() != mem_display_name {
                                                    println!(
                                                        "{} is in the channel at the start",
                                                        member.display_name());
                                                }
                                            }
                                            println!(
                                                "President: {} joined meeting voice. Meeting has started",
                                                mem_display_name);
                                            data.meeting_time.store(true, Ordering::SeqCst)
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(old_state) = old.clone() {
                    if new.channel_id.is_none() && old_state.member.is_some() {
                        if let Some(old_channel_id) = old_state.channel_id {
                            if let (Some(meeting_channel), Some(president)) =
                                (
                                    framework.user_data.config_data.channels.get(MEETING_CHANNEL),
                                    framework.user_data.config_data.guild.get(PRESIDENT)
                                ) {
                                if old_channel_id.get() == *meeting_channel
                                    && old_state.user_id.get() == *president {
                                    let mem = old_state.member.clone().unwrap();
                                    println!(
                                        "President: {} left meeting voice. Meeting has ended",
                                        mem.display_name());
                                    data.meeting_time.store(false, Ordering::SeqCst)
                                }
                            }
                        }
                    }
                }
                if data.meeting_time.load(Ordering::Relaxed) && new.channel_id.is_some() {
                    println!("{} Has joined during the meeting", new_member.display_name())
                }
            }
        },
        _ => {}
    }
    Ok(())
}
