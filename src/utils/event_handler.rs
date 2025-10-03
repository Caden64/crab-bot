use poise::serenity_prelude::FullEvent::{
    GuildMemberAddition, InteractionCreate, ReactionAdd, ReactionRemove, Ready,
};
use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateModal, EditMember, InputTextStyle, RoleId,
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
                if let Some(id) = component.guild_id {
                    if id != data.config_data.guild.main.guild_id {
                        let message = CreateInteractionResponseMessage::new()
                            .ephemeral(true)
                            .content("You are not in the right discord server");
                        component
                            .create_response(
                                ctx.http(),
                                CreateInteractionResponse::Message(message),
                            )
                            .await?;
                        return Ok(());
                    }
                } else {
                    let message = CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content("You need to be in a discord server");
                    component
                        .create_response(ctx.http(), CreateInteractionResponse::Message(message))
                        .await?;
                    return Ok(());
                }
                if component.data.custom_id == "register" {
                    println!("test");
                    let modal = CreateModal::new("custom_modal", "input").components(vec![
                        CreateActionRow::InputText(
                            CreateInputText::new(InputTextStyle::Short, "First Name", "first_name")
                                .placeholder("John")
                                .required(true)
                                .min_length(1)
                                .max_length(50),
                        ),
                        CreateActionRow::InputText(
                            CreateInputText::new(
                                InputTextStyle::Short,
                                "Last Initial",
                                "last_initial",
                            )
                            .placeholder("D")
                            .required(true)
                            .min_length(1)
                            .max_length(3),
                        ),
                        CreateActionRow::InputText(
                            CreateInputText::new(InputTextStyle::Short, "Student email", "email")
                                .placeholder("email")
                                .required(true)
                                .min_length(17)
                                .max_length(100),
                        ),
                    ]);
                    println!("test2");
                    component
                        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                        .await?
                } else {
                    let message = CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content("Unsupported Operation");
                    component
                        .create_response(ctx.http(), CreateInteractionResponse::Message(message))
                        .await?;
                    return Ok(());
                }
            }
            serenity::Interaction::Modal(modal) => {
                if modal.data.custom_id == "custom_modal" {
                    let mut first_name: Option<String> = None;
                    let mut last_initial: Option<String> = None;
                    let mut student_email: Option<String> = None;

                    for row in &modal.data.components {
                        for comp in &row.components {
                            if let serenity::all::ActionRowComponent::InputText(input) = comp {
                                if input.custom_id == "first_name" {
                                    first_name = Some(input.value.clone().unwrap_or_default());
                                } else if input.custom_id == "last_initial" {
                                    last_initial = Some(input.value.clone().unwrap_or_default());
                                } else if input.custom_id == "email" {
                                    student_email = Some(input.value.clone().unwrap_or_default());
                                }
                            }
                        }
                    }

                    let content = format!(
                        "Registration received. First: {}, Last Initial: {}, Email: {}",
                        first_name.unwrap_or_else(|| "None Given".to_string()),
                        last_initial.unwrap_or_else(|| "None Given".to_string()),
                        student_email.unwrap_or_else(|| "None Given".to_string())
                    );

                    let message = CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content(content);
                    if let Err(err) = modal
                        .create_response(&ctx.http, CreateInteractionResponse::Message(message))
                        .await
                    {
                        error!("Failed to respond to modal submit: {err:?}");
                    }
                } else {
                    let message = CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content("Unsupported Operation");
                    if let Err(err) = modal
                        .create_response(ctx.http(), CreateInteractionResponse::Message(message))
                        .await
                    {
                        error!("Failed to respond to unsupported modal: {err:?}");
                    }
                    return Ok(());
                }
            }
            _ => {}
        },

        ReactionAdd { add_reaction, .. } => {
            if let Some(guild) = add_reaction.guild_id
                && guild == data.config_data.guild.main.guild_id
            {
                println!("yay");
                println!("{}", add_reaction.emoji);
                if let Some(user) = add_reaction.user_id {
                    let mut roles = guild.member(ctx.http(), user).await?.roles;
                    roles.push(RoleId::new(1423653984748441702));

                    guild
                        .edit_member(ctx.http(), user, EditMember::new().roles(roles))
                        .await?;
                }
            } else {
                println!("no")
            }
        }
        ReactionRemove {
            removed_reaction, ..
        } => {
            let Some(guild_id) = removed_reaction.guild_id else {
                println!("no");
                return Ok(());
            };

            match (
                guild_id == data.config_data.guild.main.guild_id,
                removed_reaction.user_id,
            ) {
                (true, Some(user_id)) => {
                    let target_role = RoleId::new(1423653984748441702);

                    let member = guild_id.member(ctx.http(), user_id).await?;
                    if member.roles.contains(&target_role) {
                        let mut roles = member.roles.clone();
                        roles.retain(|r| r != &target_role);

                        guild_id
                            .edit_member(ctx.http(), user_id, EditMember::new().roles(roles))
                            .await?;
                    }
                }
                (true, None) => {
                    // No user_id in the event
                }
                (false, _) => {
                    println!("no");
                }
            }
        }
        _ => {}
    }
    Ok(())
}
