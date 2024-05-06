use futures::{future, Stream, StreamExt};

use crate::Context;
use crate::utils::config::get_config;

pub async fn college_autocomplete<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a{
   let roles = get_config().unwrap().roles;
    futures::stream::iter(roles.public.into_keys())
        .filter(move |name| future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())

}