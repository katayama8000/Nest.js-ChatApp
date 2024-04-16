use std::sync::Arc;

pub trait QueryHandler {}

pub trait HasQueryHandler {
    fn query_handler(&self) -> Arc<dyn QueryHandler + Send + Sync>;
}
