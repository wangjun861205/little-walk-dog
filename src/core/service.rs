use std::default;

use crate::core::{
    entities::{
        breed::{BreedCreate, BreedQuery},
        dog::DogUpdate,
    },
    error::Error,
    repository::Repository,
};

use super::{
    entities::{
        breed::Breed,
        dog::{Dog, DogCreate, DogQuery},
    },
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

    pub async fn create_dog(&self, owner_id: &str, dog: &DogCreate) -> Result<String, Error> {
        self.repository.create_dog(owner_id, dog).await
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

    pub async fn my_dogs(&self, owner_id: &str, page: &Pagination) -> Result<(Vec<Dog>, i64), Error> {
        self.repository
            .query_dogs(
                &DogQuery {
                    owner_id_eq: Some(owner_id.to_owned()),
                    ..default::Default::default()
                },
                page,
            )
            .await
    }

    pub async fn query_dogs(&self, query: &DogQuery, page: &Pagination) -> Result<(Vec<Dog>, i64), Error> {
        self.repository.query_dogs(query, page).await
    }

    pub async fn is_owner_of_the_dog(&self, owner_id: &str, dog_id: &str) -> Result<bool, Error> {
        self.repository
            .exists_dog(&DogQuery {
                id_eq: Some(dog_id.into()),
                owner_id_eq: Some(owner_id.into()),
                ..Default::default()
            })
            .await
    }
}
