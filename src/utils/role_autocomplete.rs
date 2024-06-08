use futures::{future, Stream, StreamExt};

use crate::utils::roles::roles;
use crate::Context;

// Gets the roles from the config and returns them
pub async fn role_autocomplete<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(roles(ctx))
        .filter(move |name| future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}
