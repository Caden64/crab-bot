use crate::checks::admin_role::admin_role;
use crate::storage::remove_user_points::remove_user_points;
use crate::{Context, Error};
use poise::serenity_prelude::User;

#[poise::command(slash_command, ephemeral, guild_only, check = "admin_role")]
pub async fn remove_points(ctx: Context<'_>, user: User, points: u16) -> Result<(), Error> {
    if remove_user_points(&user.id.get(), points).is_some() {
        ctx.reply(format!(
            "successfully removed {} points from {}",
            points, user.name
        ))
        .await?;
    } else {
        ctx.reply(format!(
            "Failed to remove {} points from {}",
            points, user.name
        ))
        .await?;
    }
    Ok(())
}
