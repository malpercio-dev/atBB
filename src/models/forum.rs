//! The Forum struct

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ForumId(pub Box<str>);

impl ForumId {
    pub fn from_str(inner: &str) -> Self {
        Self(inner.into())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Forum {
    pub id: ForumId,
    pub name: String,
    pub description: String,
}
