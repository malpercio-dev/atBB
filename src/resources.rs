//! Resources used by the application such as databases, caches,
//! or external services

mod database;
mod jetstream;

pub use database::DatabaseClient;
