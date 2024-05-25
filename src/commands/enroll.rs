use crate::checks::enroll_channel::enroll_channel;
use crate::checks::remove_role::remove_role;
use crate::storage::database_storage::save_to_json;
use crate::storage::user::User;
use crate::utils::college_autocomplete::college_autocomplete;
use crate::utils::config::{ADMIN_ROLE_ID, REMOVE_ROLE_ID};
use crate::{storage, Context, Error};
use log::error;
use poise::serenity_prelude::{CreateMessage, EditMember, Mentionable, RoleId};
use poise::CreateReply;
use serde_json::json;

#[poise::command(
    slash_command,
    guild_only,
    check = "remove_role",
    check = "enroll_channel"
)]
pub async fn enroll(
    ctx: Context<'_>,
    #[description = "First and at least last initial"] name: String,
    #[description = "Your student email"] email: String,
    #[description = "Why are you interested in cyber club"] interests: String,
    #[autocomplete = "college_autocomplete"]
    #[description = "What college do you go to"]
    university: String,
    #[description = "Would you like to occasionally receive emails"] email_distro: Option<bool>,
) -> Result<(), Error> {
    // Ensure the name input contains more than one word
    if name.split_whitespace().count() == 0 {
        ctx.reply("Need a last initial included").await?;
        return Ok(());
    }

    // Split name into first name and last initial
    let first = name.split_ascii_whitespace().next().unwrap();
    let last_initial = name
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let email_distro = email_distro.unwrap_or_default();

    // Check if the university name exists in the public roles
    if !ctx
        .data()
        .config_data
        .roles
        .public
        .contains_key(&university)
    {
        ctx.reply("Unknown university selected please try again")
            .await?;
        return Ok(());
    }
    let uni_role = ctx.data().config_data.roles.public.get(&university);
    if uni_role.is_none() {
        ctx.reply("INVALID UNIVERSITY").await?;
        return Ok(());
    }
    let uni_role = *uni_role.unwrap();

    // Retrieve some info about the guild member
    let guild_id = ctx.guild_id();
    let http = ctx.http();
    let member_id = ctx.author().id;
    let remove_role_id = ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID);

    // Formulate an error output in case anything happens
    let error_format = format!(
        "Hi {}, Something has gone wrong. The people with {} will help you!",
        ctx.author_member().await.unwrap().mention(),
        guild_id
            .unwrap()
            .roles(&ctx.http())
            .await
            .unwrap()
            .get(&RoleId::new(
                *ctx.data()
                    .config_data
                    .roles
                    .private
                    .get(ADMIN_ROLE_ID)
                    .unwrap()
            ))
            .unwrap()
            .mention()
    );

    // Try to assign new roles and nickname to the member
    match guild_id {
        Some(id) => {
            // Prepare the modification of the member
            let builder = EditMember::new()
                .roles(vec![uni_role])
                .nickname(format!("{} {}", first, last_initial));
            // Try to apply the changes
            match id.edit_member(&http, member_id, builder).await {
                Ok(member) => {
                    // Remove the remove_role if it exists
                    if let Some(role_id) = remove_role_id {
                        match member.remove_role(&http, *role_id).await {
                            Ok(_) => (),
                            Err(_) => {
                                // Handle errors by sending a message
                                ctx.defer_ephemeral().await?;
                                ctx.reply(error_format).await?;
                                return Ok(());
                            }
                        }
                    }
                }
                Err(_) => {
                    // Handle errors by sending a message
                    ctx.defer_ephemeral().await?;
                    ctx.reply(error_format).await?;
                    return Ok(());
                }
            }
        }
        None => {
            ctx.reply(error_format).await?;
            return Ok(());
        }
    };

    // Everything went fine, let's notify the user
    ctx.reply(format!("You have registered as:\nName: {} {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", first, last_initial, email, interests, university, email_distro)).await?;

    // Prepare the user data as JSON
    let user_data_json = json!({
        "user_id": ctx.author().id.get(),
        "user_name": ctx.author().name,
        "name": format!("{} {}", first, last_initial),
        "university": university,
        "email": email,
        "interests": interests,
        "email_distro": email_distro,
        "points": 0,
    });

    // Convert the JSON to the User struct
    let user_data: User = serde_json::from_value(user_data_json).unwrap();

    // Try to save the user data on a JSON file
    if let Err(e) = save_to_json(&user_data) {
        // Log any errors that happened during saving
        error!("Error saving to json {:?}", e)
    }
    Ok(())
}
