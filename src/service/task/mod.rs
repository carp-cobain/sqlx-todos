use crate::{
    domain::{Status, Task},
    repo::Repo,
    Result,
};
use std::sync::Arc;

// Task use cases
mod usecase;
use usecase::CreateTask;
use usecase::{delete_task, get_task, get_tasks, update_task};

/// A high-level API for managaing tasks.
/// This service is composed of use cases.
pub struct TaskService {
    repo: Arc<Repo>,
}

impl TaskService {
    /// Create a new task service
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    /// Create a task
    pub async fn create(&self, story_id: i32, name: String) -> Result<Task> {
        let create_task = CreateTask(self.repo.clone());
        create_task(story_id, name).await
    }

    /// Delete a task
    pub async fn delete(&self, id: i32) -> Result<()> {
        delete_task(self.repo.clone(), id).await
    }

    /// Get a task
    pub async fn get(&self, id: i32) -> Result<Task> {
        get_task(self.repo.clone(), id).await
    }

    /// Get tasks for a story
    pub async fn list(&self, story_id: i32) -> Result<Vec<Task>> {
        get_tasks(self.repo.clone(), story_id).await
    }

    /// Update a task
    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        status: Option<Status>,
    ) -> Result<Task> {
        update_task(self.repo.clone(), id, name, status).await
    }
}
