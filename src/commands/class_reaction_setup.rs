use poise::serenity_prelude::{CreateMessage, ReactionType};

use crate::{Context, Error};
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn admin_create_class_selection(ctx: Context<'_>) -> Result<(), Error> {
    let x = ctx
        .data()
        .config_data
        .roles
        .emoji
        .values()
        .map(|y| Into::<ReactionType>::into(y.clone()));
    let msg = CreateMessage::new().content("test").reactions(x);
    ctx.channel_id().send_message(ctx.http(), msg).await?;
    ctx.reply("done").await?;
    Ok(())
}
