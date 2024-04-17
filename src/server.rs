use std::sync::Arc;

use crate::{
    api::app_state::AppState,
    command::{
        command_handler::CommandHandler,
        interface::circle_repository_interface::{CircleRepositoryInterface, HasCircleRepository},
    },
    infrastructure::circle_repository::CircleRepository,
    query::query_handler::QueryHandler,
};

pub fn new() -> anyhow::Result<AppState> {
    struct CommandHandlerStruct {
        circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
    }
    impl HasCircleRepository for CommandHandlerStruct {
        fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync> {
            self.circle_repository.clone()
        }
    }
    impl CommandHandler for CommandHandlerStruct {}
    let command_handler = CommandHandlerStruct {
        circle_repository: Arc::new(CircleRepository::new()),
    };

    struct QueryHandlerStruct {}
    impl QueryHandler for QueryHandlerStruct {}
    let query_handler = QueryHandlerStruct {};
    Ok(AppState::new(
        Arc::new(command_handler),
        Arc::new(query_handler),
    ))
}
