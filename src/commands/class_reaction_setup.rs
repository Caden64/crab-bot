use poise::serenity_prelude::CreateMessage;

use crate::{Context, Error};
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn admin_create_class_selection(ctx: Context<'_>) -> Result<(), Error> {
    let msg = CreateMessage::new()
        .content("test")
        .reactions(vec!['✨', '😊', '👍']);
    ctx.channel_id().send_message(ctx.http(), msg).await?;
    Ok(())
}
