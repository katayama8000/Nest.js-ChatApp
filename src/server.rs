use std::sync::Arc;

use crate::{
    api::app_state::AppState, command::command_handler::CommandHandler,
    query::query_handler::QueryHandler,
};

pub fn new() -> anyhow::Result<AppState> {
    struct CommandHandlerStruct {}
    impl CommandHandler for CommandHandlerStruct {}
    let command_handler = CommandHandlerStruct {};

    struct QueryHandlerStruct {}
    impl QueryHandler for QueryHandlerStruct {}
    let query_handler = QueryHandlerStruct {};
    Ok(AppState::new(
        Arc::new(command_handler),
        Arc::new(query_handler),
    ))
}
