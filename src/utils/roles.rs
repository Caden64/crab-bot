use crate::Context;

pub fn roles(ctx: Context) -> Vec<String> {
    ctx.data()
        .config_data
        .roles
        .public
        .iter()
        .map(|v| v.0.to_string())
        .collect::<Vec<String>>()
}