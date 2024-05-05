use crate::{Context, Error, get_config, GUILD_ID, REMOVE_ROLE_ID};

#[poise::command(slash_command, ephemeral)]
pub async fn register(
    ctx: Context<'_>,
    name: String,
    email: String,
    interests: String,
    university: String,
    email_distro: Option<bool>,
) -> Result<(), Error> {
    let config = get_config().unwrap();

    if ctx.author().has_role(&ctx.http(), *config.guild.get(GUILD_ID).unwrap(), *config.roles.get(REMOVE_ROLE_ID).unwrap()).await.unwrap() {
        let email_distro = email_distro.unwrap_or_default();
        ctx.reply(format!("You have registered as:\nName: {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", name, email, interests, university, email_distro)).await?;
    } else {
        ctx.reply("You don't have the required permissions sorry :(") .await?;
    }
    Ok(())
}
