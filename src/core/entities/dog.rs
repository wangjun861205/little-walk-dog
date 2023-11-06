use crate::core::entities::breed::Breed;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct DogCreate {
    pub name: String,
    pub gender: String,
    pub breed: String,           // 品种
    pub birthday: DateTime<Utc>, // 生日
    pub is_sterilized: bool,     // 是否绝育
    pub introduction: String,
    pub tags: Vec<String>,
    pub portrait_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DogUpdate {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub breed: Option<String>,       // 品种
    pub birthday: Option<String>,    // 生日
    pub is_sterilized: Option<bool>, // 是否绝育
    pub introduction: Option<String>,
    pub owner_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub portrait_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DogQuery {
    pub owner_id_eq: Option<String>,
}
