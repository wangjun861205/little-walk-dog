use crate::core::{
    entities::Dog,
    repository::{DogCreate, DogQuery, DogUpdate, Pagination, Repository},
    service::Service,
};
use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    web::{Data, Json, Path, Query},
    Error, HttpRequest,
};
use serde::{Deserialize, Serialize};

use super::common::HeaderUserID;

#[derive(Debug, Serialize)]
pub struct CreateDogResult {
    pub id: String,
}

pub async fn create_dog<R>(serive: Data<Service<R>>, req: HttpRequest, Json(dog): Json<DogCreate>) -> Result<Json<CreateDogResult>, Error>
where
    R: Repository,
{
    let uid = req.headers().get("X-User-ID").ok_or(ErrorForbidden("not allowed"))?.to_str().map_err(ErrorForbidden)?;
    serive.create_dog(uid, &dog).await.map(|id| Json(CreateDogResult { id })).map_err(ErrorInternalServerError)
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

pub async fn my_dogs<R>(service: Data<Service<R>>, HeaderUserID(uid): HeaderUserID, Query(pagination): Query<Pagination>) -> Result<Json<Vec<Dog>>, Error>
where
    R: Repository,
{
    service.my_dogs(&uid, Some(pagination)).await.map_err(ErrorInternalServerError).map(Json)
}

pub async fn dogs<R>(service: Data<Service<R>>, Query(query): Query<DogQuery>) -> Result<Json<Vec<Dog>>, Error>
where
    R: Repository,
{
    let dogs = service.query_dogs(&query).await.map_err(ErrorInternalServerError)?;
    Ok(Json(dogs))
}

#[derive(Debug, Deserialize)]
pub struct IsOwnerOfTheDogReq {
    id: String,
    owner_id: String,
}

#[derive(Debug, Serialize)]
pub struct IsOwnerOfTheDogResp {
    is_owner: bool,
}

pub async fn is_owner_of_the_dog<R>(service: Data<Service<R>>, Query(query): Query<IsOwnerOfTheDogReq>) -> Result<Json<IsOwnerOfTheDogResp>, Error>
where
    R: Repository,
{
    let is_owner = service.is_owner_of_the_dog(&query.owner_id, &query.id).await.map_err(ErrorInternalServerError)?;
    Ok(Json(IsOwnerOfTheDogResp { is_owner }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateDogPortraitReq {
    portrait_id: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateDogPortraitResp {
    has_updated: bool,
}

pub async fn update_dog_portrait<R>(service: Data<Service<R>>, dog_id: Path<(String,)>, Json(query): Json<UpdateDogPortraitReq>) -> Result<Json<UpdateDogPortraitResp>, Error>
where
    R: Repository,
{
    let has_updated = service.update_dog_portrait(&dog_id.as_ref().0, &query.portrait_id).await.map_err(ErrorInternalServerError)?;
    Ok(Json(UpdateDogPortraitResp { has_updated }))
}
