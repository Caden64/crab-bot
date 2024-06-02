use poise::serenity_prelude as serenity;
use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Message, Ready, VoiceStateUpdate};
use poise::serenity_prelude::{CacheHttp, CreateMessage, GuildId, Mentionable};

use crate::utils::HandleVoiceStateUpdate::handle_voice_state_update;
use crate::{Data, Error};
use crate::utils::config::READING_CHANNEL;

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
            let gd = new_member.guild_id.get();
            if data.config_data.guild.main.guild_id == gd {
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
            if new_message.channel_id == *data.config_data.channels.get(READING_CHANNEL).unwrap() {
                
            }
        }
        _ => {}
    }
    Ok(())
}
