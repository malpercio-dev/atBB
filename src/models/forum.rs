//! The Forum struct

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Forum {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}
