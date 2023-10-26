use crate::core::{
    entities::dog::{Dog, DogCreate, DogQuery, DogUpdate},
    repository::{Pagination, Repository},
    service::Service,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Path, Query},
    Error,
};
use serde::Serialize;

use super::common::{HeaderUserID, ListResp};

#[derive(Debug, Serialize)]
pub struct CreateDogResult {
    pub id: String,
}

pub async fn create_dog<R>(serive: Data<Service<R>>, Json(dog): Json<DogCreate>) -> Result<Json<CreateDogResult>, Error>
where
    R: Repository,
{
    serive.create_dog(&dog).await.map(|id| Json(CreateDogResult { id })).map_err(ErrorInternalServerError)
}

#[derive(Debug, Serialize)]
pub struct UpdateDogResult {
    pub updated: bool,
}
pub async fn update_dog<R>(service: Data<Service<R>>, id: Path<(String,)>, Json(dog): Json<DogUpdate>) -> Result<Json<UpdateDogResult>, Error>
where
    R: Repository,
{
    service.update_dog(&id.0, &dog).await.map_err(ErrorInternalServerError).map(|updated| Json(UpdateDogResult { updated }))
}

pub async fn my_dogs<R>(service: Data<Service<R>>, HeaderUserID(uid): HeaderUserID, Query(page): Query<Pagination>) -> Result<Json<(Vec<Dog>, i64)>, Error>
where
    R: Repository,
{
    service.my_dogs(&uid, &page).await.map_err(ErrorInternalServerError).map(Json)
}

pub async fn dogs<R>(service: Data<Service<R>>, Query(query): Query<DogQuery>, Query(page): Query<Pagination>) -> Result<Json<ListResp<Dog>>, Error>
where
    R: Repository,
{
    let (dogs, total) = service.query_dogs(&query, &page).await.map_err(ErrorInternalServerError)?;
    Ok(Json(ListResp::new(dogs, total)))
}
