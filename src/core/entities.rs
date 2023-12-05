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

// 性别
#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    Other,
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Other
    }
}

// 狗狗
#[derive(Debug, Serialize, Deserialize)]
pub struct Dog {
    pub id: String,
    pub name: String,
    pub gender: Gender,
    pub breed: Breed,            // 品种
    pub birthday: DateTime<Utc>, // 生日
    pub is_sterilized: bool,     // 是否绝育
    pub introduction: String,
    pub owner_id: String,
    pub tags: Vec<String>,
    pub portrait_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
