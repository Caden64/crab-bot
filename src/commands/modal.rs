use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
struct MyModal {
    first_input: String,
    second_input: Option<String>,
}

#[poise::command(prefix_command, slash_command)]
pub async fn component_modal(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal")
                .label("Open modal")
                .style(poise::serenity_prelude::ButtonStyle::Success),
        ])];

        poise::CreateReply::default()
            .content("Click the button below to open the modal")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<MyModal>(ctx, mci, None, None).await?;
        println!("Got data: {:?}", data);
    }
    Ok(())
}