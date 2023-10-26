use crate::{
    core::{
        entities::breed::{Breed, BreedCreate, BreedQuery},
        repository::{Pagination, Repository},
        service::Service,
    },
    handlers::common::ListResp,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Query},
    Error,
};

pub(crate) async fn create_breed<R>(service: Data<Service<R>>, Json(breed): Json<BreedCreate>) -> Result<String, Error>
where
    R: Repository,
{
    service.create_breed(breed).await.map_err(ErrorInternalServerError)
}

pub(crate) async fn breeds<R>(service: Data<Service<R>>, Query(query): Query<BreedQuery>, Query(page): Query<Pagination>) -> Result<Json<ListResp<Breed>>, Error>
where
    R: Repository,
{
    let (breeds, total) = service.query_breeds(&query, &page).await.map_err(ErrorInternalServerError)?;
    Ok(Json(ListResp::new(breeds, total)))
}
