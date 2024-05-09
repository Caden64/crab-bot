use log::error;
use poise::serenity_prelude::EditMember;
use crate::{Context, Error};
use crate::utils::college_autocomplete::college_autocomplete;
use crate::checks::remove_role::remove_role;
use crate::utils::config::REMOVE_ROLE_ID;

#[poise::command(
slash_command, guild_only,
check = "remove_role"
)]
pub async fn enroll(
    ctx: Context<'_>,
    name: String,
    email: String,
    interests: String,
    #[autocomplete = "college_autocomplete"]
    university: String,
    email_distro: Option<bool>,
) -> Result<(), Error> {
    let email_distro = email_distro.unwrap_or_default();
    if !ctx.data().config_data.roles.public.contains_key(&university) { 
        ctx.reply("Unknown university selected please try again").await?;
        return Ok(()) 
    }
    let uni_role = ctx.data().config_data.roles.public.get(&university);
    if uni_role.is_none() {
        ctx.reply("INVALID UNIVERSITY").await?;
        return Ok(())
    }
    let uni_role = *uni_role.unwrap();
    
    let builder = EditMember::new().roles(vec![uni_role]).nickname(name.clone());
    if let Err(e) = ctx.guild_id().unwrap().edit_member(&ctx.http(), ctx.author().id, builder).await.unwrap().remove_role(&ctx.http(), *ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID).unwrap()).await {
        error!("ERROR REMOVING ROLE {:?}", e)
    };
    ctx.reply(format!("You have registered as:\nName: {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", name, email, interests, university, email_distro)).await?;
    Ok(())
}
