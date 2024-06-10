use crate::utils::config::REMOVE_ROLE_ID;
use crate::{Context, Error};
use poise::serenity_prelude::{CreateMessage, EditMember, Mentionable};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, ephemeral)]
pub async fn acknowledge(ctx: Context<'_>) -> Result<(), Error> {
    if ctx
        .guild_id()
        .unwrap()
        .member(&ctx.http(), ctx.author().id.get())
        .await
        .unwrap()
        .roles
        .is_empty()
    {
        let ctx_uuid = ctx.id();

        // Prepare and send a reply with an "Acknowledge" button
        let reply = {
            let components = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(format!("{ctx_uuid}"))
                    .style(serenity::ButtonStyle::Primary)
                    .label("Acknowledge"),
            ])];

            CreateReply::default()
                .content("Please acknowledge to enroll")
                .components(components)
        };

        ctx.send(reply).await?;

        // Start component interaction collection, looking for a button press of the specific button by comparing custom id's
        while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
            .timeout(std::time::Duration::from_secs(120))
            .filter(move |mci| mci.data.custom_id == ctx_uuid.to_string())
            .await
        {
            // Create a message to send to the user
            let enroll_message = CreateMessage::new().content(format!(
                "{}, You may now enroll in {}",
                mci.user.mention(),
                ctx.guild().unwrap().name
            ));

            // Define user role changes
            let edit_user = EditMember::new().roles(vec![*ctx
                .data()
                .config_data
                .roles
                .private
                .get(REMOVE_ROLE_ID)
                .unwrap()]);

            // Check if user doesn't have any other roles
            // Change user roles in the guild
            ctx.guild_id()
                .unwrap()
                .edit_member(ctx.http(), mci.user.id, edit_user)
                .await
                .unwrap();
            // Send DM to the user
            mci.user
                .create_dm_channel(&ctx.http())
                .await
                .unwrap()
                .send_message(&ctx.http(), enroll_message)
                .await
                .unwrap();
            // Send acknowledgment reply
            mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                .await?
        }
    } else {
        ctx.say("Sorry you can already use the /enroll command or have a university role already")
            .await?;
    }
    Ok(())
}
