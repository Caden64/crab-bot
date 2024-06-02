use std::sync::atomic::Ordering;

use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CacheHttp;
use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Message, Ready, VoiceStateUpdate};

use crate::{Data, Error};
use crate::utils::config::MEETING_CHANNEL;

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
            println!("{} Joined Server", new_member.user.name);
        }
        VoiceStateUpdate { new, old } => {
            // If member entered a voice channel
            if let Some(new_member) = new.member.as_ref() {
                if old.is_none() {
                    if let Some(new_channel_id) = new.channel_id.as_ref() {
                        if let (Some(meeting_channel), president) = (
                            // Load configuration data
                            framework
                                .user_data
                                .config_data
                                .channels
                                .get(MEETING_CHANNEL),
                            framework.user_data.config_data.guild.main.PRESIDENT,
                        ) {
                            // If the member is the president, and they joined the meeting channel
                            if new_channel_id.get() == *meeting_channel
                                && new_member.user.id.get() == president
                            {
                                let mem_display_name = new_member.display_name();
                                // Fetch all channels from guild
                                if let Ok(channels) =
                                    new.guild_id.unwrap().channels(ctx.http.http()).await
                                {
                                    // If the meeting channel exists
                                    if let Some(new_channel) = channels.get(new_channel_id) {
                                        // Fetch all members from the channel
                                        if let Ok(members) =
                                            new_channel.members(ctx.cache.clone().as_ref())
                                        {
                                            // Fetch all members who aren't the president
                                            for member in members {
                                                if member.display_name() != mem_display_name {
                                                    println!(
                                                        "{} is in the channel at the start",
                                                        member.display_name()
                                                    );
                                                }
                                            }
                                            // Notify that the meeting has started
                                            println!(
                                                "President: {} joined meeting voice. Meeting has started",
                                                mem_display_name);
                                            // Set meeting time flag to true
                                            data.meeting_time.store(true, Ordering::SeqCst)
                                        }
                                    }
                                }
                            }
                        }
                    }
                // If a member left a voice channel
                } else if let Some(old_state) = old.clone() {
                    // If the member is not in any voice channel currently
                    if new.channel_id.is_none() && old_state.member.is_some() {
                        // The channel that the member left
                        if let Some(old_channel_id) = old_state.channel_id {
                            if let (Some(meeting_channel), president) = (
                                // Load configuration data
                                framework
                                    .user_data
                                    .config_data
                                    .channels
                                    .get(MEETING_CHANNEL),
                                framework.user_data.config_data.guild.main.PRESIDENT,
                            ) {
                                // If the member is the president, and they left the meeting channel
                                if old_channel_id.get() == *meeting_channel
                                    && old_state.user_id.get() == president
                                {
                                    let mem = old_state.member.clone().unwrap();
                                    // Notify that the meeting has ended
                                    println!(
                                        "President: {} left meeting voice. Meeting has ended",
                                        mem.display_name()
                                    );
                                    // Set meeting time flag to false
                                    data.meeting_time.store(false, Ordering::SeqCst)
                                }
                            }
                        }
                    }
                }
                // Print message if a member joins during the meeting time
                if data.meeting_time.load(Ordering::Relaxed) && new.channel_id.is_some() {
                    println!(
                        "{} Has joined during the meeting",
                        new_member.display_name()
                    )
                }
            }
        }
        // Fallback for other types of event
        Message { new_message } => {
            // new_message.channel_id
        }
        _ => {}
    }
    Ok(())
}
