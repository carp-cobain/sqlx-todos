use super::sql;
use super::Repo;
use crate::{domain::Story, Error, Result};
use futures_util::TryStreamExt;
use sqlx::{postgres::PgRow, FromRow, Row};
use stripmargin::StripMargin;

/// Map sqlx rows to story domain objects.
impl FromRow<'_, PgRow> for Story {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}

// Extend repo with queries related to stories.
impl Repo {
    /// Select a story by id
    pub async fn fetch_story(&self, id: i32) -> Result<Story> {
        tracing::debug!("fetch_story: id={}", id);

        let query = sql::story::FETCH.strip_margin();
        tracing::debug!("sql: {}", query);

        let maybe_story = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(self.db_ref())
            .await?;

        match maybe_story {
            Some(story) => Ok(story),
            None => Err(Error::not_found(format!("story not found: {}", id))),
        }
    }

    /// Select a page of stories.
    pub async fn list_stories(&self, page_id: i32, page_size: i32) -> Result<Vec<Story>> {
        tracing::debug!("list_stories: page_id={}", page_id);

        let query = sql::story::LIST.strip_margin();
        tracing::debug!("sql: {}", query);

        let mut result_set = sqlx::query(&query)
            .bind(page_id)
            .bind(page_size)
            .fetch(self.db_ref());

        let mut stories = Vec::with_capacity(page_size as usize);

        while let Some(row) = result_set.try_next().await? {
            let story = Story::from_row(&row)?;
            stories.push(story);
        }

        Ok(stories)
    }

    /// Insert a new story
    pub async fn create_story(&self, name: String) -> Result<Story> {
        tracing::debug!("create_story: name={}", name);

        let query = sql::story::CREATE.strip_margin();
        tracing::debug!("sql: {}", query);

        let story = sqlx::query_as(&query)
            .bind(name)
            .fetch_one(self.db_ref())
            .await?;

        Ok(story)
    }

    /// Update story name
    pub async fn update_story(&self, id: i32, name: String) -> Result<Story> {
        tracing::debug!("update_story: id={}, name={}", id, name);

        let query = sql::story::UPDATE.strip_margin();
        tracing::debug!("sql: {}", query);

        let story = sqlx::query_as(&query)
            .bind(name)
            .bind(id)
            .fetch_one(self.db_ref())
            .await?;

        Ok(story)
    }

    /// Delete a story and its tasks.
    pub async fn delete_story(&self, id: i32) -> Result<u64> {
        tracing::debug!("delete_story: id={}", id);

        let mut tx = self.db.begin().await?;

        let query = sql::task::DELETE_BY_STORY.strip_margin();
        tracing::debug!("sql: {}", query);
        sqlx::query(&query).bind(id).execute(&mut *tx).await?;

        let query = sql::story::DELETE.strip_margin();
        tracing::debug!("sql: {}", query);
        let result = sqlx::query(&query).bind(id).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(result.rows_affected())
    }

    /// Check whether a story exists.
    pub async fn story_exists(&self, id: i32) -> bool {
        tracing::debug!("story_exists: id={}", id);

        let query = sql::story::EXISTS.strip_margin();
        tracing::debug!("sql: {}", query);

        let result = sqlx::query(&query).bind(id).fetch_one(self.db_ref()).await;
        if let Ok(row) = result {
            row.try_get::<bool, _>("exists").unwrap_or_default()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::tests;

    use testcontainers::{runners::AsyncRunner, ImageExt};
    use testcontainers_modules::postgres::Postgres;

    #[ignore]
    #[tokio::test]
    async fn integration_test() {
        // Set up postgres test container backed repo
        let image = Postgres::default().with_tag("16-alpine");
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(pool);

        // Create story
        let name = "Books To Read".to_string();
        let story = repo.create_story(name.clone()).await.unwrap();
        assert_eq!(name, story.name);

        // Assert the story exists
        assert!(repo.story_exists(story.id).await);

        // Query stories page
        let stories = repo.list_stories(i32::MAX, 10).await.unwrap();
        assert_eq!(stories.len(), 1);

        // Update the name
        let updated_name = "Books".to_string();
        repo.update_story(story.id, updated_name).await.unwrap();

        // Fetch and verify new name
        let story = repo.fetch_story(story.id).await.unwrap();
        assert_eq!(story.name, "Books");

        // Delete the story
        let rows = repo.delete_story(story.id).await.unwrap();
        assert_eq!(rows, 1);

        // Assert story was deleted
        assert!(!repo.story_exists(story.id).await);
    }
}
