use std::sync::Arc;

// #[async_trait::async_trait]
pub trait CommandHandler {}

pub trait HasCommandHandler {
    fn command_handler(&self) -> Arc<dyn CommandHandler + Send + Sync>;
}
