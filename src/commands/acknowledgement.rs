use poise::{serenity_prelude as serenity, CreateReply};
use poise::serenity_prelude::{CreateMessage, Mentionable};
use crate::{Context, Error};
use crate::checks::enroll_channel::enroll_channel;

#[poise::command(slash_command)]
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
        .filter(move |mci| mci.data.custom_id == ctx_uuid.to_string()).await {
        let enroll_message = CreateMessage::new().content(format!("{}, You may now enroll in {}", mci.user.mention(), ctx.guild().unwrap().name));
        mci.user.create_dm_channel(&ctx.http()).await.unwrap().send_message(&ctx.http(), enroll_message).await.unwrap();
        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?
    } 
    
    Ok(())
}