use poise::serenity_prelude::FullEvent::{GuildMemberAddition, Ready};
use poise::serenity_prelude as serenity;
use crate::{Data, Error, REMOVE_ROLE_ID};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    _: &Data,
) -> Result<(), Error>{
    match event {
        Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        },
        GuildMemberAddition { new_member, .. } => {
            println!("{} Joined Server", new_member.user.name);
            let default_role_id = *framework.user_data.config_data.roles.private.get(REMOVE_ROLE_ID).expect("UNABLE TO GET REMOVE ROLE ID");
            new_member.add_role(&ctx.http, default_role_id).await?;
            println!("{} got default role", new_member.user.name)
        },
        _ => {}
    }
    Ok(())
}
