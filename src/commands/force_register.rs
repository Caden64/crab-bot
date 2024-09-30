use crate::checks::admin_role::admin_role;
use crate::{Context, Error};
use poise::serenity_prelude::{EditMember, Mentionable, RoleId, User};
use serde_json::json;
use crate::storage::save_user::save_to_json;
use crate::utils::config::{ADMIN_ROLE_ID, REMOVE_ROLE_ID};

// use poise to generate a decent help menu
#[poise::command(slash_command, guild_only, check = "admin_role")]
pub async fn force_register(
    ctx: Context<'_>,
    user: User,
    first: String,
    last_initial: String,
    email: String,
    role: String,
    email_list: Option<bool>
) -> Result<(), Error> {
    let email_list = email_list.unwrap_or(false);
    let guild_id = ctx.guild_id();
    let http = ctx.http();
    let member_id = user.id;
    let remove_role_id = ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID);


    let uni_role = ctx.data().config_data.roles.public.get(&role);
    if uni_role.is_none() {
        ctx.reply("INVALID UNIVERSITY").await?;
        return Ok(());
    }
    let uni_role = *uni_role.unwrap();

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
    let user_data: crate::storage::user::User = serde_json::from_value(user_data_json).unwrap();

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
