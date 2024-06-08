use crate::checks::{
    check_user_exists::check_user_exists, check_user_not_exists::check_user_not_exists,
};
use crate::{Context, Error};

#[poise::command(
    slash_command,
    ephemeral,
    check = "check_user_not_exists",
    check = "check_user_exists"
)]
pub async fn edit_user(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
