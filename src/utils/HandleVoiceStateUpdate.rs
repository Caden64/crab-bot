use crate::utils::config::MEETING_CHANNEL;
use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CacheHttp, VoiceState};
use std::sync::atomic::Ordering;

// If member entered a voice channel
pub async fn handle_voice_state_update(
    new: &VoiceState,
    old: &Option<VoiceState>,
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) {
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
                        if let Ok(channels) = new.guild_id.unwrap().channels(ctx.http.http()).await
                        {
                            // If the meeting channel exists
                            if let Some(new_channel) = channels.get(new_channel_id) {
                                // Fetch all members from the channel
                                if let Ok(members) = new_channel.members(ctx.cache.clone().as_ref())
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
                                        mem_display_name
                                    );
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
