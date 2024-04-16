use axum::Router;

use crate::{command::command_handler::HasCommandHandler, query::query_handler::HasQueryHandler};

pub fn route<T: Clone + HasCommandHandler + HasQueryHandler + Send + Sync + 'static>(
    shared_state: T,
) -> Router {
    Router::new()
}
