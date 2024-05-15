use poise::{serenity_prelude as serenity, CreateReply};
use poise::serenity_prelude::{CreateMessage, EditMember, Mentionable};
use crate::{Context, Error};
use crate::utils::config::REMOVE_ROLE_ID;

#[poise::command(slash_command, ephemeral)]
pub async  fn acknowledgement(ctx: Context<'_>) -> Result<(), Error> {
    let ctx_uuid = ctx.id();
    
    // Prepare and send a reply with an "Acknowledge" button
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(format!("{ctx_uuid}"))
                .style(serenity::ButtonStyle::Primary)
                .label("Acknowledge")
        ])];
        
        CreateReply::default().content("Please acknowledge to enroll").components(components)
    };
    
    ctx.send(reply).await?;

    // Start component interaction collection, looking for a button press of the specific button by comparing custom id's
    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .filter(move |mci| mci.data.custom_id == ctx_uuid.to_string()).await {
        
        // Create a message to send to the user
        let enroll_message = CreateMessage::new().content(format!("{}, You may now enroll in {}", mci.user.mention(), ctx.guild().unwrap().name));
        
        // Define user role changes
        let edit_user = EditMember::new().roles(vec![*ctx.data().config_data.roles.private.get(REMOVE_ROLE_ID).unwrap()]);
        
        // Check if user doesn't have any other roles
        if ctx.guild_id().unwrap().member(&ctx.http(), mci.user.id).await.unwrap().roles.len() == 0{
            // Change user roles in the guild
            ctx.guild_id().unwrap().edit_member(ctx.http(), mci.user.id, edit_user).await.unwrap();
            // Send DM to the user
            mci.user.create_dm_channel(&ctx.http()).await.unwrap().send_message(&ctx.http(), enroll_message).await.unwrap();
        }
        
        // Send acknowledgment reply
        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?
    } 
    
    Ok(())
}