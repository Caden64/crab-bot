use futures::{future, Stream, StreamExt};

use crate::Context;

pub async fn college_autocomplete<'a>(ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a{
    let roles = ctx.data().config_data.roles.public.iter().map(|v | v.0.to_string() ).collect::<Vec<String>>();
    futures::stream::iter(roles)
        .filter(move |name| future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())

}