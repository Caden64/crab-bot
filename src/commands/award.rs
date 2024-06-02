use poise::serenity_prelude::User;
use crate::{Context, Error};
use crate::storage::edit_database::edit_user_points;

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn award(
   ctx: Context<'_>, 
   users: User,
   points: u16,
) -> Result<(), Error>{
   edit_user_points(&users.id.get(), points); 
   ctx.reply(format!("Going to add {} points to {}", points, users.name)).await?;
   Ok(()) 
}