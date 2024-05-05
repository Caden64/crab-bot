use crate::{Context, Error};

#[poise::command(slash_command, ephemeral, hide_in_help)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
    Ok(())
}
