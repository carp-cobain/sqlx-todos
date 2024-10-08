use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub struct Story {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing)]
    pub seqno: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
