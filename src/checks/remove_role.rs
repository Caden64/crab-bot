use crate::{Context, Error, GUILD_ID, REMOVE_ROLE_ID};

pub async fn remove_role(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx.author().has_role(&ctx.http(), *ctx.data().config_data.guild.get(GUILD_ID).unwrap(), *ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID).unwrap()).await.unwrap() {
        return Ok(true)
    }
    ctx.say("You don't have the proper permissions").await?;
    Ok(false)
}
