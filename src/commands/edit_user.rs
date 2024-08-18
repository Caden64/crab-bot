use crate::storage::user::EditableUser;
use crate::{
    checks::check_user_exists::check_user_exists, storage::get_user::get_user, Context, Error,
};
use poise::{serenity_prelude as serenity, CreateReply};
use crate::storage::save_user::save_to_json;

#[poise::command(slash_command, ephemeral, guild_only, check = "check_user_exists")]
pub async fn edit_user(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(user) = get_user(&ctx.author().id.get()) {
        let ctx_uuid = ctx.id();
        let mut editable_user = user.clone();
        // Prepare and send a reply with an "Acknowledge" button
        let reply = {
            let components = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(format!("{ctx_uuid}"))
                    .style(serenity::ButtonStyle::Primary)
                    .label("Edit user"),
            ])];

            CreateReply::default()
                .content("Click button to edit user")
                .components(components)
        };

        ctx.send(reply).await?;

        while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
            .timeout(std::time::Duration::from_secs(120))
            .filter(move |mci| mci.data.custom_id == ctx_uuid.to_string())
            .await 
        {
            let data = poise::execute_modal_on_component_interaction::<EditableUser>(ctx, mci, None, None).await?;
            if let Some(data) = data {
                if let Some(name) = data.name {
                    editable_user.name = name
                }
                if let Some(email) = data.email {
                    editable_user.email = email
                }
                if let Some(thm_username) = data.thm_username {
                    editable_user.thm_username = thm_username
                }
                if save_to_json(&editable_user).is_err() {
                    ctx.reply("Unable to save changes to json").await?; 
                } else {
                    ctx.reply("saved changes to json").await?;
                }
                
            }
        }
    } else {
        ctx.reply("Sorry it seems like you have not registered yet!")
            .await?;
    }
    Ok(())
}
