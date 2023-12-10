use crate::core::entities::{Breed, Category, Dog};
use crate::core::error::Error;
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use nb_serde_query::Array;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub limit: i64,
    pub skip: i64,
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BreedQuery {
    pub id: Option<String>,
    pub category: Option<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DogCreate {
    pub name: String,
    pub gender: String,
    pub breed: BreedQuery,       // 品种
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
    pub breed: Option<BreedQuery>,   // 品种
    pub birthday: Option<String>,    // 生日
    pub is_sterilized: Option<bool>, // 是否绝育
    pub introduction: Option<String>,
    pub owner_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub portrait_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DogQuery {
    pub id: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub owner_id: Option<String>,
    pub pagination: Option<Pagination>,
}

pub trait Repository {
    async fn create_breed(&self, breed: &BreedCreate) -> Result<String, Error>;
    async fn delete_breed(&self, id: &str) -> Result<bool, Error>;
    async fn query_breeds(&self, query: &BreedQuery) -> Result<(Vec<Breed>, i64), Error>;
    async fn create_dog(&self, owner_id: &str, dog: &DogCreate) -> Result<String, Error>;
    async fn delete_dog(&self, id: &str) -> Result<bool, Error>;
    async fn update_dog(&self, id: &str, dog: &DogUpdate) -> Result<bool, Error>;
    async fn query_dogs(&self, query: &DogQuery) -> Result<Vec<Dog>, Error>;
    async fn exists_dog(&self, query: &DogQuery) -> Result<bool, Error>;
}
