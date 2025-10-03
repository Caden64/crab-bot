use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateMessage};

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn admin_create_modal(ctx: Context<'_>) -> Result<(), Error> {
    let components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("register")
            .label("Register")
            .style(serenity::ButtonStyle::Success),
    ])];
    ctx.channel_id()
        .send_message(ctx.http(), CreateMessage::new().components(components))
        .await?;
    ctx.say("setup").await?;
    Ok(())
}
