use poise::serenity_prelude::FullEvent::{
    GuildMemberAddition, InteractionCreate, ReactionAdd, ReactionRemove, Ready,
};
use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateModal, EditMember, InputTextStyle, ReactionType,
    ReactionType::Custom, RoleId, SelectMenu,
};
use poise::serenity_prelude::{
    CacheHttp, CreateInteractionResponseFollowup, CreateMessage, CreateSelectMenu, Mentionable,
};
use tracing::error;

use crate::storage::save_user::save_to_json;
use crate::storage::user::User;
use crate::utils::config::{EmojiType, REMOVE_ROLE_ID};
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
                if modal.data.custom_id == "custom_modal"
                    && modal.guild_id.is_some()
                    && modal
                        .guild_id
                        .unwrap()
                        .member(ctx.http(), modal.user.id.get())
                        .await?
                        .roles
                        .contains(&RoleId::new(
                            data.config_data
                                .roles
                                .private
                                .get(REMOVE_ROLE_ID)
                                .unwrap()
                                .to_owned(),
                        ))
                {
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
                        "Registration received.\nFirst: {},\nLast Initial: {},\nEmail: {}",
                        first_name
                            .clone()
                            .unwrap_or_else(|| "None Given".to_string()),
                        last_initial
                            .clone()
                            .unwrap_or_else(|| "None Given".to_string()),
                        student_email
                            .clone()
                            .unwrap_or_else(|| "None Given".to_string())
                    );

                    let _ = save_to_json(&User {
                        user_id: modal.user.id.get(),
                        user_name: modal.user.name.to_ascii_lowercase(),
                        name: format!("{} {}", first_name.unwrap(), last_initial.unwrap()),
                        role: "".into(),
                        email: student_email.unwrap(),
                        points: 0,
                    });

                    let roles = modal
                        .guild_id
                        .unwrap()
                        .member(ctx.http(), modal.user.id)
                        .await?;
                    roles
                        .remove_role(
                            ctx.http(),
                            RoleId::new(
                                data.config_data
                                    .roles
                                    .private
                                    .get(REMOVE_ROLE_ID)
                                    .unwrap()
                                    .to_owned(),
                            ),
                        )
                        .await?;

                    let message = CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content(content);
                    if let Err(err) = modal
                        .create_response(&ctx.http, CreateInteractionResponse::Message(message))
                        .await
                    {
                        error!("Failed to respond to modal submit: {err:?}");
                    }
                } else if modal.guild_id.is_none() {
                    let message = CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content("Must be done in a discord server");
                    if let Err(err) = modal
                        .create_response(ctx.http(), CreateInteractionResponse::Message(message))
                        .await
                    {
                        error!("Failed to respond to `Must be done in a discord server`: {err:?}");
                    }
                    return Ok(());
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
            let Some(guild_id) = add_reaction.guild_id else {
                // not from a guild
                return Ok(());
            };

            match (
                guild_id == data.config_data.guild.main.guild_id,
                add_reaction.user_id,
            ) {
                (true, Some(user_id)) => {
                    for (_, emoji_cfg) in data.config_data.roles.emoji.iter().enumerate() {
                        // Match against your config enum
                        match &emoji_cfg.1.emoji {
                            // Unicode emoji mapping
                            EmojiType::Str(expected) => {
                                if add_reaction.emoji.unicode_eq(expected) {
                                    let member = guild_id.member(ctx.http(), user_id).await?;
                                    let target_role = RoleId::new(emoji_cfg.1.role);

                                    if !member.roles.contains(&target_role) {
                                        let mut roles = member.roles.clone();
                                        roles.push(target_role);

                                        guild_id
                                            .edit_member(
                                                ctx.http(),
                                                user_id,
                                                EditMember::new().roles(roles),
                                            )
                                            .await?;
                                    }
                                }
                            }
                            // Custom emoji mapping by ID
                            EmojiType::Id(expected_emoji_id) => {
                                if let Custom { id, .. } = add_reaction.emoji {
                                    // Compare your stored ID to the Custom id
                                    if id.get() == *expected_emoji_id {
                                        let member = guild_id.member(ctx.http(), user_id).await?;
                                        let target_role = RoleId::new(emoji_cfg.1.role);

                                        if !member.roles.contains(&target_role) {
                                            let mut roles = member.roles.clone();
                                            roles.push(target_role);

                                            guild_id
                                                .edit_member(
                                                    ctx.http(),
                                                    user_id,
                                                    EditMember::new().roles(roles),
                                                )
                                                .await?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                (true, None) => {
                    // No user_id in the event
                }
                (false, _) => {
                    // Not your main guild
                }
            }

            return Ok(());
        }
        ReactionRemove {
            removed_reaction, ..
        } => {
            let Some(guild_id) = removed_reaction.guild_id else {
                return Ok(());
            };

            match (
                guild_id == data.config_data.guild.main.guild_id,
                removed_reaction.user_id,
            ) {
                (true, Some(user_id)) => {
                    for (_, emoji) in data.config_data.roles.emoji.iter().enumerate() {
                        // check if it's an ID for a custom
                        if emoji.0.is_ascii() {
                            println!("{}", emoji.0)
                        }
                        match &emoji.1.emoji {
                            EmojiType::Str(string_data) => {
                                if removed_reaction.emoji.unicode_eq(&string_data) {
                                    let member = guild_id.member(ctx.http(), user_id).await?;
                                    let target_role = RoleId::new(emoji.1.role);
                                    if member.roles.contains(&target_role) {
                                        let mut roles = member.roles.clone();
                                        roles.retain(|r| r != &target_role);

                                        guild_id
                                            .edit_member(
                                                ctx.http(),
                                                user_id,
                                                EditMember::new().roles(roles),
                                            )
                                            .await?;
                                    }
                                }
                            }
                            EmojiType::Id(_discord_id) => {}
                        }
                    }
                }
                (true, None) => {
                    // No user_id in the event
                }
                (false, _) => {
                }
            }
        }
        _ => {}
    }
    Ok(())
}
