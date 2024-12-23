use crate::checks::admin_role::admin_role;
use crate::{Context, Error};
use poise::serenity_prelude::User;

// use poise to generate a decent help menu
#[poise::command(slash_command, ephemeral, guild_only, check = "admin_role")]
pub async fn user(ctx: Context<'_>, user: User) -> Result<(), Error> {
    ctx.reply(format!(
        "server_name: {}\nglobal name: {}\nuser_id: {}\nmfa: {}",
        user.name,
        user.global_name.unwrap_or(
            "Unable to get global name"
                .parse()
                .unwrap_or(String::from("Unable to get global name"))
        ),
        user.id,
        user.mfa_enabled
    ))
    .await?;
    Ok(())
}
