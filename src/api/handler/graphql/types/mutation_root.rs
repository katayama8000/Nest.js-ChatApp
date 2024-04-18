pub mod create_circle;
use super::create_circle_input::CreateCircleInput;
use async_graphql::Context;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_circle(self, context: &Context<'_>, input: CreateCircleInput) -> String {
        self::create_circle::resolve(context, input);
        "ok".to_string()
    }
}
