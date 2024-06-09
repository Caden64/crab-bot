use crate::{Context, Error};
use crate::checks::check_user_exists::check_user_exists;
use crate::storage::get_user::get_user;

#[poise::command(
    slash_command, 
    ephemeral,
    guild_only,
    check = "check_user_exists"
)]
pub async fn edit_user(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(user) = get_user(&ctx.author().id.get())  {
        
    } else {
        ctx.reply("Sorry it seems like you have not registered yet!").await?; 
    }
    Ok(())
}