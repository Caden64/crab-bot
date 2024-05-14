use std::sync::atomic::Ordering;
use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Ready, VoiceStateUpdate};
use poise::serenity_prelude as serenity;
use crate::{Data, Error};
use crate::utils::config::{MEETING_CHANNEL, PRESIDENT};

pub async fn event_handler(
    _: &serenity::Context,
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
        VoiceStateUpdate { new, old} => {
            if new.member.is_some() &&
                old.is_none() &&
                new.channel_id.is_some() &&
                new.channel_id.unwrap().get() == *framework.user_data.config_data.channels.get(MEETING_CHANNEL).unwrap() &&
                new.user_id.get() == *framework.user_data.config_data.guild.get(PRESIDENT).unwrap() {
                let mem = new.member.clone().unwrap();
                println!("President: {} joined meeting voice. Meeting has started", mem.display_name());
                data.meeting_time.store(true, Ordering::SeqCst)

            } else if old.is_some() &&
                new.channel_id.is_none() &&
                old.clone().unwrap().channel_id.unwrap().get() == *framework.user_data.config_data.channels.get(MEETING_CHANNEL).unwrap() &&
                old.clone().unwrap().user_id.get() == *framework.user_data.config_data.guild.get(PRESIDENT).unwrap() && old.clone().unwrap().member.is_some(){
                let mem =  old.clone().unwrap().member.clone().unwrap();
                println!("President: {} left meeting voice. Meeting has started", mem.display_name());
                data.meeting_time.store(false, Ordering::SeqCst)

            } else if data.meeting_time.load(Ordering::Relaxed) && new.channel_id.is_some() && new.member.is_some(){
               println!("{} Has joined during the meeting", new.clone().member.unwrap().display_name())
            }
        },
        _ => {}
    }
    Ok(())
}
