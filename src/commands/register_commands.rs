use crate::{Context, Error};

#[poise::command(prefix_command, hide_in_help, guild_only, ephemeral, slash_command, owners_only)]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
