//! Resources used by the application such as databases, caches,
//! or external services

mod database;

pub use database::DatabaseClient;
pub use database::DatabaseKind;
pub use database::DatabaseUrl;
