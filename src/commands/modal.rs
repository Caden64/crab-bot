use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, guild_only)]
pub async fn admin_create_modal(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("register")
                .label("Register")
                .style(serenity::ButtonStyle::Success),
        ])];
        poise::CreateReply::default()
            .content("Click Register")
            .components(components)
    };

    ctx.send(reply).await?;
    Ok(())
}
