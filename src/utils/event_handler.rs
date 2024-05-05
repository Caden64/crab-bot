use poise::serenity_prelude::FullEvent::GuildMemberAddition;
use poise::serenity_prelude as serenity;
use crate::{Data, Error, REMOVE_ROLE_ID};
use crate::utils::config::get_config;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, Data, Error>,
    _: &Data,
) -> Result<(), Error>{
    if let GuildMemberAddition { new_member, .. } = event {
        println!("{} Joined Server", new_member.user.name);
        let config = get_config().unwrap();
        let default_role_id = *config.roles.get(REMOVE_ROLE_ID).expect("UNABLE TO GET REMOVE ROLE ID");
        new_member.add_role(&ctx.http, default_role_id).await?;
        println!("{} got default role", new_member.user.name)
    };
    Ok(())
}
