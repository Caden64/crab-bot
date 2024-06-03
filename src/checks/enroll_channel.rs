use crate::utils::config::ENROLL_CHANNEL;
use crate::{Context, Error};

// makes sure the command was sent in the enroll channel
pub async fn enroll_channel(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx.guild_channel().await.unwrap().id.get()
        == *ctx.data().config_data.channels.get(ENROLL_CHANNEL).unwrap()
    {
        return Ok(true);
    }
    Ok(false)
}
