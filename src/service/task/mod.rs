use super::UseCase;
use crate::{
    domain::{Status, Task},
    repo::Repo,
    Result,
};
use std::sync::Arc;

// Use case mods
mod create_task;
mod delete_task;
mod get_task;
mod get_tasks;
mod update_task;

// Use cases
use create_task::CreateTask;
use delete_task::DeleteTask;
use get_task::GetTask;
use get_tasks::GetTasks;
use update_task::UpdateTask;

/// A high-level API for managaing tasks.
pub struct TaskService {
    create_task: CreateTask,
    delete_task: DeleteTask,
    get_task: GetTask,
    get_tasks: GetTasks,
    update_task: UpdateTask,
}

impl TaskService {
    /// Create a new task service
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            create_task: CreateTask::new(repo.clone()),
            delete_task: DeleteTask::new(repo.clone()),
            get_task: GetTask::new(repo.clone()),
            update_task: UpdateTask::new(repo.clone()),
            get_tasks: GetTasks::new(repo),
        }
    }

    /// Create a task
    pub async fn create(&self, story_id: i32, name: String) -> Result<Task> {
        let args = (story_id, name);
        self.create_task.execute(args).await
    }

    /// Delete a task
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.delete_task.execute(id).await
    }

    /// Get a task
    pub async fn get(&self, id: i32) -> Result<Task> {
        self.get_task.execute(id).await
    }

    /// Get tasks for a story
    pub async fn list(&self, story_id: i32) -> Result<Vec<Task>> {
        self.get_tasks.execute(story_id).await
    }

    /// Update a task
    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        status: Option<Status>,
    ) -> Result<Task> {
        let args = (id, name, status);
        self.update_task.execute(args).await
    }
}
