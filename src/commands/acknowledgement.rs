use poise::{serenity_prelude as serenity, CreateReply};
use poise::serenity_prelude::{CreateMessage, EditMember, Mentionable};
use crate::{Context, Error};
use crate::utils::config::REMOVE_ROLE_ID;

#[poise::command(slash_command, ephemeral)]
pub async  fn acknowledgement(ctx: Context<'_>) -> Result<(), Error> {
    let ctx_uuid = ctx.id();
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(format!("{ctx_uuid}"))
                .style(serenity::ButtonStyle::Primary)
                .label("Acknowledge")
        ])];
        
        CreateReply::default().content("Please acknowledge to enroll").components(components)
    };
    
    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .filter( move |mci| mci.data.custom_id == ctx_uuid.to_string()).await {
        let enroll_message = CreateMessage::new().content(format!("{}, You may now enroll in {}", mci.user.mention(), ctx.guild().unwrap().name));
        let edit_user = EditMember::new().roles(vec![*ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID).unwrap()]);
        if mci.user.has_role(&ctx.http(), ctx.guild_id().unwrap(), *ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID).unwrap()).await.is_ok() {
            ctx.guild_id().unwrap().edit_member(ctx.http(), mci.user.id, edit_user).await.unwrap();
            mci.user.create_dm_channel(&ctx.http()).await.unwrap().send_message(&ctx.http(), enroll_message).await.unwrap();
        }
        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?
    } 
    
    Ok(())
}