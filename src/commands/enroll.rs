use crate::checks::enroll_channel::enroll_channel;
use crate::checks::remove_role::remove_role;
use crate::storage::save_user::save_to_json;
use crate::storage::user::User;
use crate::utils::config::{ADMIN_ROLE_ID, REMOVE_ROLE_ID};
use crate::utils::role_autocomplete::role_autocomplete;
use crate::{Context, Error};
use poise::serenity_prelude::{EditMember, Mentionable, RoleId};
use regex::Regex;
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
    #[autocomplete = "role_autocomplete"]
    #[description = "What role best describes you"]
    role: String,
    #[description = "Would you like to occasionally receive emails"] email_distro: Option<bool>,
) -> Result<(), Error> {
    // Ensure the name input contains more than one word
    if name.split_whitespace().count() == 0 {
        ctx.reply("Need a last initial included").await?;
        return Ok(());
    }
    let email_regex = Regex::new("(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|\"(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21\\x23-\\x5b\\x5d-\\x7f]|\\\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])*\")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21-\\x5a\\x53-\\x7f]|\\\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])+)\\])").unwrap();
    if !email_regex.is_match(&email) {
        ctx.reply("Invalid email format").await?;
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
    if !ctx.data().config_data.roles.public.contains_key(&role) {
        ctx.reply("Unknown university selected please try again")
            .await?;
        return Ok(());
    }
    let uni_role = ctx.data().config_data.roles.public.get(&role);
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
    ctx.reply(format!("You have registered as:\nName: {} {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", first, last_initial, email, interests, role, email_distro)).await?;

    // Prepare the user data as JSON
    let user_data_json = json!({
        "user_id": ctx.author().id.get(),
        "user_name": ctx.author().name,
        "name": format!("{} {}", first, last_initial),
        "university": role,
        "email": email,
        "interests": interests,
        "email_distro": email_distro,
        "points": 0,
    });

    // Convert the JSON to the User struct
    let user_data: User = serde_json::from_value(user_data_json).unwrap();

    // Try to save the user data on a JSON file
    if save_to_json(&user_data).is_err() {
        // Log any errors that happened during saving
        let error_format = format!(
            "Hi {}, Something has gone wrong. The people with {} will get you set up to earn points!",
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
        ctx.reply(error_format).await?;
    }
    Ok(())
}
