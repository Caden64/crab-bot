use crate::utils::config::ADMIN_ROLE_ID;
use crate::{Context, Error};

pub async fn admin_role(ctx: Context<'_>) -> Result<bool, Error> {
    if let Some(role) = ctx.data().config_data.roles.private.get(ADMIN_ROLE_ID) {
        match ctx
            .author()
            .has_role(
                &ctx.http(),
                ctx.data().config_data.guild.main.guild_id,
                *role,
            )
            .await
        {
            Ok(has_role) => {
                if has_role {
                    return Ok(true);
                } else {
                    println!("Member did not have role");
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Error: Failed to get role");
    }
    ctx.say("You don't have the proper permissions").await?;
    Ok(false)
}
