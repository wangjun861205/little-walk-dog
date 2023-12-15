use std::default;

use crate::core::{
    error::Error,
    repository::{BreedCreate, BreedQuery, DogCreate, DogQuery, DogUpdate, Repository},
};

use super::{
    entities::{Breed, Dog},
    repository::Pagination,
};

pub struct Service<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> Service<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub async fn create_breed(&self, breed: BreedCreate) -> Result<String, Error> {
        self.repository.create_breed(&breed).await
    }

    pub async fn delete_breed(&self, id: &str) -> Result<bool, Error> {
        self.repository.delete_breed(id).await
    }

    pub async fn query_breeds(&self, query: &BreedQuery) -> Result<(Vec<Breed>, i64), Error> {
        self.repository.query_breeds(query).await
    }

    pub async fn create_dog(&self, dog: &DogCreate) -> Result<Dog, Error> {
        self.repository.create_dog(dog).await
    }

    pub async fn update_dog_portrait(&self, id: &str, portrait_id: &str) -> Result<bool, Error> {
        self.repository
            .update_dog(
                id,
                &DogUpdate {
                    portrait_id: Some(portrait_id.to_owned()),
                    ..default::Default::default()
                },
            )
            .await
    }

    pub async fn update_dog(&self, id: &str, dog: &DogUpdate) -> Result<bool, Error> {
        self.repository.update_dog(id, dog).await
    }

    pub async fn my_dogs(&self, owner_id: &str, pagination: Option<Pagination>) -> Result<Vec<Dog>, Error> {
        self.repository
            .query_dogs(&DogQuery {
                owner_id: Some(owner_id.to_owned()),
                pagination,
                ..default::Default::default()
            })
            .await
    }

    pub async fn query_dogs(&self, query: &DogQuery) -> Result<Vec<Dog>, Error> {
        self.repository.query_dogs(query).await
    }

    pub async fn is_owner_of_the_dog(&self, owner_id: &str, dog_id: &str) -> Result<bool, Error> {
        self.repository
            .exists_dog(&DogQuery {
                id: Some(dog_id.into()),
                owner_id: Some(owner_id.into()),
                ..Default::default()
            })
            .await
    }
}
