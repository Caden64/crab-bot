use crate::storage::user_exists::user_exists;
use crate::{Context, Error};

pub async fn check_user_exists(ctx: Context<'_>) -> Result<bool, Error> {
    println!("CHECK USER EXISTS CALLED");
    Ok(user_exists(&ctx.author().id.get()))
}
