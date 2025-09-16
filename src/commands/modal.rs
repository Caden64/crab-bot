use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[derive(poise::Modal)]
#[derive(Debug)]
struct MeModal {
    data1: String
}

#[poise::command(slash_command, guild_only)]
pub async fn modal(ctx: Context<'_>) -> Result<(), Error>{
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal").label("Open Modal").style(serenity::ButtonStyle::Success)
        ])];
        poise::CreateReply::default()
            .content("It's Modal Time")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await {
        let data = poise::execute_modal_on_component_interaction::<MeModal>(ctx, mci, None, None).await?;
        println!("{:?}", data)
    }
    Ok(())
}

