use crate::Result;

pub mod fs;

// TODO:
// pub mod s3;
// pub mod gcs;

/// Read and write binary objects.
#[async_trait::async_trait]
pub trait Storage<Key>: Send + Sync {
    /// Read bytes for a key
    async fn read(&self, key: Key) -> Result<Vec<u8>>;

    /// Write bytes and return a key
    async fn write(&self, bytes: &[u8]) -> Result<Key>;

    /// Delete bytes for a key
    async fn delete(&self, key: Key) -> Result<()>;
}
