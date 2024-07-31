#![feature(unboxed_closures)]
#![feature(async_fn_traits)]
#![feature(async_closure)]

pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod repo;
pub mod service;

/// Expose error at the top level
pub use error::Error;

/// Project level result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
