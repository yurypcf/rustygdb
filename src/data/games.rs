use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: usize,
    pub name: Option<String>,
    pub created_at: Option<String>,
}
