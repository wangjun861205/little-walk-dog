use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Category {
    Small,
    Medium,
    Large,
    Giant,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Category::Small => "Small",
                Category::Medium => "Medium",
                Category::Large => "Large",
                Category::Giant => "Giant",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub category: Category,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreedCreate {
    pub category: Category,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreedUpdate {
    pub name: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BreedQuery {
    pub category_eq: Option<Category>,
}
