use crate::core::{
    entities::dog::{Dog, DogCreate, DogQuery, DogUpdate},
    repository::{Pagination, Repository},
    service::Service,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError},
    web::{Data, Header, Json, Path, Query},
    Error, HttpRequest,
};
use serde::Serialize;

use super::common::{HeaderUserID, ListResp};

#[derive(Debug, Serialize)]
pub struct CreateDogResult {
    pub id: String,
}

pub async fn create_dog<R>(serive: Data<Service<R>>, req: HttpRequest, Json(dog): Json<DogCreate>) -> Result<Json<CreateDogResult>, Error>
where
    R: Repository,
{
    let uid = req.headers().get("X-User-ID").ok_or(ErrorForbidden("not allowed"))?.to_str().map_err(|e| ErrorForbidden(e))?;
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
