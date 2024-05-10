use poise::serenity_prelude::{CreateMessage, EditMember, Mentionable, RoleId};
use crate::{Context, Error};
use crate::utils::college_autocomplete::college_autocomplete;
use crate::checks::remove_role::remove_role;
use crate::checks::enroll_channel::enroll_channel;
use crate::utils::config::{ADMIN_ROLE_ID, REMOVE_ROLE_ID};

#[poise::command(
slash_command, guild_only,
check = "remove_role", check = "enroll_channel"
)]
pub async fn enroll(
    ctx: Context<'_>,
    #[description = "First and at least last initial"]
    name: String,
    #[description = "Your student email"]
    email: String,
    #[description = "Why are you interested in cyber club"]
    interests: String,
    #[autocomplete = "college_autocomplete"]
    #[description = "What college do you go to"]
    university: String,
    #[description = "Would you like to occasionally receive emails"]
    email_distro: Option<bool>,
) -> Result<(), Error> {
    if !name.contains(" ") {
       ctx.reply("Need a last initial included").await?; 
        return Ok(())
    }
    let first = name.split_ascii_whitespace().next().unwrap();
    let last_initial = name.split_ascii_whitespace().nth(1).unwrap().chars().next().unwrap();
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
    
    let guild_id = ctx.guild_id();
    let http = ctx.http();
    let member_id = ctx.author().id;
    let remove_role_id = ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID);
    let error_builder = CreateMessage::new().content(format!("Hi {}, Something has gone wrong. The people with {} will help you!", ctx.author_member().await.unwrap().mention(), guild_id.unwrap().roles(&ctx.http()).await.unwrap().get(&RoleId::new(*ctx.data().config_data.roles.private.get(ADMIN_ROLE_ID).unwrap())).unwrap().mention()));
    match guild_id {
        Some(id) => {
            let builder = EditMember::new().roles(vec![uni_role]).nickname(format!("{} {}", first, last_initial));
            match id.edit_member(&http, member_id, builder).await {
                Ok(member) => {
                    if let Some(role_id) = remove_role_id {
                        match member.remove_role(&http, *role_id).await {
                            Ok(_) => (),
                            Err(_) => {
                                ctx.guild_channel().await.unwrap().send_message(&ctx.http(), error_builder).await?;
                                return Ok(());
                            }
                        }
                    }
                },
                Err(_) => {
                    ctx.guild_channel().await.unwrap().send_message(&ctx.http(), error_builder).await?;
                    return Ok(());
                }
            }
        },
        None => {
            ctx.guild_channel().await.unwrap().send_message(&ctx.http(), error_builder).await?;
            return Ok(());
        }
    };
    ctx.reply(format!("You have registered as:\nName: {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", format!("{} {}", first, last_initial), email, interests, university, email_distro)).await?;
    Ok(())
}
