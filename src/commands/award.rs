use crate::storage::edit_database::add_user_points;
use crate::{Context, Error};
use poise::serenity_prelude::User;

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn award(ctx: Context<'_>, users: User, points: u16) -> Result<(), Error> {
    if add_user_points(&users.id.get(), points).is_some() {
        ctx.reply(format!("Added {} points to {}", points, users.name))
            .await?;
    }
    ctx.reply(format!("Failed to add {} points to {}", points, users.name))
        .await?;
    Ok(())
}
