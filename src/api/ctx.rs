use crate::{
    config::Config,
    repo::{StoryRepo, TaskRepo},
    service::{story::StoryService, task::TaskService},
};
use sqlx::postgres::PgPool;
use std::sync::Arc;

/// API context
#[derive(Clone)]
pub struct Ctx {
    /// Config
    pub config: Arc<Config>,
    /// Story service
    pub stories: Arc<StoryService>,
    /// Task service
    pub tasks: Arc<TaskService>,
}

impl Ctx {
    /// Create a new api context.
    pub fn new(config: Arc<Config>, db: Arc<PgPool>) -> Self {
        // Repos
        let story_repo = Arc::new(StoryRepo::new(db.clone()));
        let task_repo = Arc::new(TaskRepo::new(db));

        // Services (organize/group use cases by domain).
        let task_service = TaskService::new(task_repo, story_repo.clone());
        let story_service = StoryService::new(story_repo);

        // Ctx
        Self {
            config,
            stories: Arc::new(story_service),
            tasks: Arc::new(task_service),
        }
    }
}
