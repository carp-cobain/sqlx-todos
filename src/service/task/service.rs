use crate::{
    domain::{Status, Task},
    repo::Repo,
    Error, Result,
};
use std::sync::Arc;

// Task use cases
use super::usecase::{delete_task, get_task, get_tasks};
use super::usecase::{CreateTask, UpdateTask};

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
    pub async fn create(
        &self,
        story_id: i32,
        name: String,
        status_opt: Option<Status>,
    ) -> Result<Task> {
        let create_task = CreateTask(self.repo.clone());
        match status_opt {
            Some(status) => create_task(story_id, name, status).await,
            None => create_task(story_id, name).await,
        }
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
        name_opt: Option<String>,
        status_opt: Option<Status>,
    ) -> Result<Task> {
        // Create use case
        let update_task = UpdateTask(self.repo.clone());
        // Apply updates if provided or return an error.
        // This is an example of the variadic nature of `AsyncFnOnce`.
        match (name_opt, status_opt) {
            // Update both name and status
            (Some(name), Some(status)) => update_task(id, name, status).await,
            // Update name
            (Some(name), None) => update_task(id, name).await,
            // Update status
            (None, Some(status)) => update_task(id, status).await,
            // Error case
            (None, None) => Err(Error::invalid_args("no task update provided")),
        }
    }
}
