use poise::serenity_prelude::{CreateMessage, ReactionType};

use crate::utils::config::EmojiType;
use crate::{Context, Error};
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn admin_create_class_selection(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("done").await?;
    let items: Vec<_> = ctx
        .data()
        .config_data
        .roles
        .emoji
        .values()
        .cloned()
        .collect();

    let reactions_to_add: Vec<ReactionType> = items
        .iter()
        .cloned()
        .map(Into::<ReactionType>::into)
        .collect();

    let mut reaction_message_context: String = Default::default();
    for data in items.iter() {
        match data.emoji.clone() {
            EmojiType::Id(_) => {
                reaction_message_context += &format!(
                    "\n<:{}> {}\n",
                    Into::<ReactionType>::into(data.clone()).as_data(),
                    data.message
                )
                .to_owned();
            }
            EmojiType::Str(emoji_str) => {
                reaction_message_context +=
                    &format!("\n{} {}\n", emoji_str, data.message).to_owned();
            }
        }
    }
    let msg = CreateMessage::new()
        .content(format!(
            "Role Selection\n--------------{}",
            reaction_message_context
        ))
        .reactions(reactions_to_add);
    ctx.channel_id().send_message(ctx.http(), msg).await?;
    Ok(())
}
