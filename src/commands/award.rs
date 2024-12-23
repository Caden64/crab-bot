use crate::checks::admin_role::admin_role;
use crate::storage::add_user_points::add_user_points;
use crate::{Context, Error};
use poise::serenity_prelude::User;

#[poise::command(slash_command, guild_only, ephemeral, check = "admin_role")]
pub async fn award(ctx: Context<'_>, users: User, points: u16) -> Result<(), Error> {
    if add_user_points(&users.id.get(), points).is_some() {
        ctx.reply(format!("Added {} points to {}", points, users.name))
            .await?;
    } else {
        ctx.reply(format!("Failed to add {} points to {}", points, users.name))
            .await?;
    }
    Ok(())
}
