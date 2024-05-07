use crate::{Context, Error};
use crate::utils::college_autocomplete::college_autocomplete;
use crate::checks::remove_role::remove_role;

#[poise::command(
slash_command, ephemeral,
check = "remove_role"
)]
pub async fn register(
    ctx: Context<'_>,
    name: String,
    email: String,
    interests: String,
    #[autocomplete = "college_autocomplete"]
    university: String,
    email_distro: Option<bool>,
) -> Result<(), Error> {
    let email_distro = email_distro.unwrap_or_default();
    if !ctx.data().config_data.roles.public.contains_key(&university) { 
        ctx.reply("Unknown university selected please try again").await?;
        return Ok(()) 
    }
    ctx.reply(format!("You have registered as:\nName: {}\nEmail: {}\nInterests: {}\nUniversity: {}\nAdd to Email Distro: {}", name, email, interests, university, email_distro)).await?;
    Ok(())
}
