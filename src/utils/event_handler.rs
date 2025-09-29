use poise::serenity_prelude::FullEvent::{GuildMemberAddition, InteractionCreate, Ready};
use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal,
    InputTextStyle,
};
use poise::serenity_prelude::{CacheHttp, CreateMessage, Mentionable};
use tracing::error;

use crate::{Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        // Bot successfully logged in
        Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }

        // New member joined the server
        GuildMemberAddition { new_member, .. } => {
            let member_guild = new_member.guild_id.get();
            if data.config_data.guild.main.guild_id == member_guild {
                let join_message = CreateMessage::new().content(format!(
                    "Welcome to {}, {}! Thanks for joining us! 🎉",
                    ctx.http().get_guild(new_member.guild_id).await?.name,
                    new_member.user.mention(),
                ));
                new_member
                    .user
                    .create_dm_channel(ctx.http.http())
                    .await?
                    .send_message(ctx.http.http(), join_message)
                    .await?;
            }
            println!(
                "{} Joined Server {} ",
                new_member.user.name,
                ctx.http().get_guild(new_member.guild_id).await?.name
            );
        }

        InteractionCreate { interaction, .. } => match interaction {
            serenity::Interaction::Component(component) => {
                if component.data.custom_id == "open_modal" {
                    let modal = CreateModal::new("custom_modal", "input").components(vec![
                        CreateActionRow::InputText(
                            CreateInputText::new(InputTextStyle::Short, "field one", "field_one")
                                .placeholder("Type stuff")
                                .min_length(1)
                                .max_length(100),
                        ),
                    ]);

                    if let Err(err) = component
                        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                        .await
                    {
                        error!("Failed to show modal {:?}", err)
                    }
                }
            }
            serenity::Interaction::Modal(modal) => {
                if modal.data.custom_id == "custom_modal" {
                    let mut user_text: Option<String> = None;

                    for row in &modal.data.components {
                        for comp in &row.components {
                            if let serenity::all::ActionRowComponent::InputText(input) = comp {
                                if input.custom_id == "field_one" {
                                    user_text = Some(input.value.clone().unwrap_or_default());
                                }
                            }
                        }
                    }
                    println!("{}", user_text.unwrap_or("None Given".to_owned()));
                    if let Err(err) = modal
                        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
                        .await
                    {
                        error!("Failed to respond to modal submit: {err:?}");
                    }
                }
            }
            _ => {}
        },

        _ => {}
    }
    Ok(())
}
