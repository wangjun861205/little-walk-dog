use crate::core::{
    entities::dog::{Dog, DogCreate, DogQuery, DogUpdate},
    repository::{Pagination, Repository},
    service::Service,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError},
    web::{Data, Header, Json, Path, Query},
    Error, FromRequest, HttpRequest,
};
use serde::{Deserialize, Serialize};

use super::common::{HeaderUserID, ListResp};
use std::future::Future;
use std::pin::Pin;

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

impl FromRequest for DogQuery {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let mut query = DogQuery::default();
        let id_eq = Option::<String>::from_request(req, payload);
        let owner_id_eq = Option::<String>::from_request(req, payload);
        match req
            .query_string()
            .split("&")
            .filter(|&pair| pair.starts_with("id_in"))
            .map(|pair| pair.split("=").nth(1).ok_or(ErrorBadRequest(format!("invalid query param: {}", pair))).map(|v| v.to_owned()))
            .collect::<Result<Vec<String>, Error>>()
        {
            Ok(id_in) => {
                if id_in.is_empty() {
                    return Box::pin(async move {
                        query.id_eq = id_eq.await?.filter(|v| !v.is_empty());
                        query.owner_id_eq = owner_id_eq.await?.filter(|v| !v.is_empty());
                        Ok(query)
                    });
                }
                Box::pin(async move {
                    query.id_eq = id_eq.await?.filter(|v| !v.is_empty());
                    query.owner_id_eq = owner_id_eq.await?.filter(|v| !v.is_empty());
                    query.id_in = Some(id_in);
                    Ok(query)
                })
            }
            Err(e) => Box::pin(async move { Err(e) }),
        }
    }
}

pub async fn dogs<R>(service: Data<Service<R>>, Query(query): Query<DogQuery>, Query(page): Query<Pagination>) -> Result<Json<ListResp<Dog>>, Error>
where
    R: Repository,
{
    let (dogs, total) = service.query_dogs(&query, &page).await.map_err(ErrorInternalServerError)?;
    Ok(Json(ListResp::new(dogs, total)))
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
