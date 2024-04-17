use serde::Deserialize;

use crate::domain::aggregate::{
    circle::Circle,
    member::Member,
    value_object::{grade::Grade, major::Major},
};

use super::interface::circle_repository_interface::HasCircleRepository;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait CommandHandler: HasCircleRepository {
    async fn create_circle(&self, input: CreateCircleInput) -> anyhow::Result<CreateCircleOutput> {
        let grade = Grade::try_from(input.owner_grade)?;
        let major = Major::from(input.owner_major.as_str());
        let owner = Member::new(input.owner_name, input.owner_age, grade, major);
        let circle = Circle::new(input.circle_name, owner, input.capacity)?;
        let circle_id = circle.id.clone();
        self.circle_repository()
            .create(&circle)
            .map(|_| CreateCircleOutput {
                circle_id: circle_id.into(),
                owner_id: circle.owner.id.into(),
            })
    }
}

pub trait HasCommandHandler {
    fn command_handler(&self) -> Arc<dyn CommandHandler + Send + Sync>;
}

// ----- move this code to somewhere else -----
#[derive(Debug, Deserialize)]
pub struct CreateCircleInput {
    pub circle_name: String,
    pub capacity: usize,
    pub owner_name: String,
    pub owner_age: usize,
    pub owner_grade: usize,
    pub owner_major: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCircleOutput {
    pub circle_id: usize,
    pub owner_id: usize,
}
