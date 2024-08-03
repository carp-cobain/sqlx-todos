use crate::{domain::Story, repo::Repo, Result};
use futures_util::TryFutureExt;
use std::ops::Deref;
use std::sync::Arc;

/// A high-level API for managaing stories.
/// This service is a standard service that does NOT rely on use cases.
/// This layout is cleaner for simple services.
pub struct StoryService {
    repo: Arc<Repo>,
}

impl StoryService {
    /// Create a new story service
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    /// Delete a story
    pub async fn delete(&self, id: i32) -> Result<()> {
        tracing::debug!("delete: id={}", id);
        self.fetch_story(id)
            .and_then(|_| self.delete_story(id))
            .await
            .map(|_| ())
    }

    /// Get a page of stories
    pub async fn list(&self, page_id: i32, page_size: i32) -> Result<(i32, Vec<Story>)> {
        tracing::debug!("list: page_id={}, page_size={}", page_id, page_size);
        let stories = self.list_stories(page_id, page_size).await?;
        let next_page = stories.last().map(|s| s.id - 1).unwrap_or_default();
        Ok((next_page, stories))
    }

    /// Update a story
    pub async fn update(&self, id: i32, name: String) -> Result<Story> {
        tracing::debug!("update: id={}, name={}", id, name);
        self.fetch_story(id)
            .and_then(|_| self.update_story(id, name))
            .await
    }
}

// Allow calls directly to the inner repo.
impl Deref for StoryService {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}
