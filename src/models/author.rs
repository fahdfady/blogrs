use serde::{Deserialize, Serialize};

#[Derive(Debug, Serialize, Deserialize)]
pub struct Author {
    id: u64,
    name: String,
    email: String,
}
