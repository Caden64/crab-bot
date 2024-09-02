use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn inline_choice(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("wow")
        .await?;

    Ok(())
}
