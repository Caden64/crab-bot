use crate::{Context, Error, get_config, GUILD_ID, REMOVE_ROLE_ID};

pub async fn remove_role(ctx: Context<'_>) -> Result<bool, Error> {
    let config = get_config().unwrap();

    if ctx.author().has_role(&ctx.http(), *config.guild.get(GUILD_ID).unwrap(), *config.roles.private.get(REMOVE_ROLE_ID).unwrap()).await.unwrap() {
        return Ok(true)
    }
    ctx.say("You don't have the proper permissions").await?;
    Ok(false)
}
