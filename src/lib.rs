pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod repo;
pub mod service;

/// Expose error at the top level
pub use error::Error;

/// Project level result type
pub type Result<T> = std::result::Result<T, Error>;
