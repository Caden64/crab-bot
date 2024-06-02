
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CacheHttp;
use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Message, Ready, VoiceStateUpdate};

use crate::{Data, Error};
use crate::utils::HandleVoiceStateUpdate::handle_voice_state_update;

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
            println!("{} Joined Server {} ", new_member.user.name, ctx.http().get_guild(new_member.guild_id).await?.name);
        }
        VoiceStateUpdate { new, old } => {
            handle_voice_state_update(new, old, ctx, framework, data).await;
        }
        // Fallback for other types of event
        Message { new_message } => {
            // new_message.channel_id
        }
        _ => {}
    }
    Ok(())
}
