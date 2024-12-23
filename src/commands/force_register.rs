use crate::checks::admin_role::admin_role;
use crate::storage::save_user::save_to_json;
use crate::utils::config::{ADMIN_ROLE_ID, REMOVE_ROLE_ID};
use crate::{Context, Error};
use poise::serenity_prelude::{EditMember, Mentionable, RoleId, User};
use serde_json::json;

// use poise to generate a decent help menu
#[poise::command(slash_command, guild_only, check = "admin_role")]
pub async fn force_register(
    ctx: Context<'_>,
    user: User,
    first: String,
    last_initial: String,
    email: String,
    role: String,
    email_list: Option<bool>,
) -> Result<(), Error> {
    let email_list = email_list.unwrap_or(false);
    let guild_id = ctx.guild_id();
    let http = ctx.http();
    let member_id = user.id;
    let remove_role_id = ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID);

    let uni_role = ctx.data().config_data.roles.public.get(&role);
    if uni_role.is_none() {
        ctx.reply("INVALID ROLE").await?;
        return Ok(());
    }
    let uni_role = match ctx.data().config_data.roles.public.get(&role) {
        Some(role) => *role,
        None => {
            ctx.reply("INVALID ROLE").await?;
            return Ok(());
        }
    };

    let error_format = match (
        ctx.author_member().await,
        guild_id,
        ctx.data().config_data.roles.private.get(ADMIN_ROLE_ID),
    ) {
        (Some(author_member), Some(guild_id), Some(admin_role)) => match guild_id
            .roles(&ctx.http())
            .await
        {
            Ok(roles) => match roles.get(&RoleId::new(*admin_role)) {
                Some(role) => format!(
                    "Hi {}, Something has gone wrong. The people with {} will help you!",
                    author_member.mention(),
                    role.mention()
                ),
                None => "An error occurred while retrieving the admin role.".to_string(),
            },
            Err(_) => "An error occurred while retrieving the roles from the guild.".to_string(),
        },
        _ => "An error occurred while preparing the error message.".to_string(),
    };

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
    ctx.reply(format!("{} have registered as:\nName: {} {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", user.name, first, last_initial, email, "admin registered", role, email_list)).await?;

    // Prepare the user data as JSON
    let user_data_json = json!({
        "user_id": ctx.author().id.get(),
        "user_name": ctx.author().name,
        "name": format!("{} {}", first, last_initial),
        "role": role,
        "email": email,
        "interests": "admin registered",
        "email_distro": email_list,
        "points": 0,
        "thm_username": ""
    });

    // Convert the JSON to the User struct
    let user_data: crate::storage::user::User = match serde_json::from_value(user_data_json) {
        Ok(data) => data,
        Err(_) => {
            ctx.reply(error_format).await?;
            return Ok(());
        }
    };

    // Try to save the user data on a JSON file
    if save_to_json(&user_data).is_err() {
        // Log any errors that happened during saving
        ctx.reply(error_format).await?;
    }
    Ok(())
}
