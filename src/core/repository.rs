use crate::core::entities::{
    breed::{Breed, BreedCreate, BreedQuery},
    dog::{Dog, DogCreate, DogUpdate},
};
use crate::core::error::Error;
use serde::{Deserialize, Serialize};

use crate::core::entities::dog::DogQuery;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub size: i64,
}

pub trait Repository {
    async fn create_breed(&self, breed: &BreedCreate) -> Result<String, Error>;
    async fn delete_breed(&self, id: &str) -> Result<bool, Error>;
    async fn query_breeds(&self, query: &BreedQuery, page: &Pagination) -> Result<(Vec<Breed>, i64), Error>;
    async fn create_dog(&self, owner_id: &str, dog: &DogCreate) -> Result<String, Error>;
    async fn delete_dog(&self, id: &str) -> Result<bool, Error>;
    async fn update_dog(&self, id: &str, dog: &DogUpdate) -> Result<bool, Error>;
    async fn query_dogs(&self, query: &DogQuery, page: &Pagination) -> Result<(Vec<Dog>, i64), Error>;
}
