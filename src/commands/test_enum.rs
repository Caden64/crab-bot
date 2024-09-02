use crate::{Context, Error};
use crate::roles::Roles;

#[poise::command(slash_command)]
pub async fn inline_choice(
    ctx: Context<'_>,
    role: Roles
) -> Result<(), Error> {
    ctx.say(format!("selected {} Wow", role))
        .await?;

    Ok(())
}
