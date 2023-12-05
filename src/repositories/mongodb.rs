use std::ops::Deref;

use mongodb::{
    bson::{doc, from_document, oid::ObjectId, Document},
    Database,
};

use crate::core::{
    entities::{Breed, Dog},
    error::Error,
    repository::{BreedCreate, BreedQuery, DogCreate, DogQuery, DogUpdate, Repository},
};

use mongodb::options::FindOptions;

use futures::TryStreamExt;

use chrono::Local;

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

impl Repository for MongoDB {
    async fn create_breed(&self, breed: &BreedCreate) -> Result<String, Error> {
        let now = Local::now();
        let d = doc! {
            "name": &breed.name,
            "category": &breed.category.to_string(),
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339(),
        };
        let res = self
            .db
            .collection::<Document>("breeds")
            .insert_one(d, None)
            .await
            .map_err(|e| Error::new("failed to create breed").with_cause(e))?;
        res.inserted_id
            .as_object_id()
            .ok_or(Error::new("failed to create breed").with_cause("invalid inserted id"))
            .map(|id| id.to_string())
    }

    async fn create_dog(&self, owner_id: &str, dog: &DogCreate) -> Result<String, Error> {
        let now = Local::now();
        let dog = doc! {
            "name": &dog.name,
            "gender": &dog.gender,
            "breed": &dog.breed.id.as_ref().ok_or(Error::new("breed cannot be null"))?,
            "birthday": &dog.birthday.to_rfc3339(),
            "is_sterilized": &dog.is_sterilized,
            "introduction": &dog.introduction,
            "owner_id": owner_id,
            "tags": &dog.tags,
            "portrait_id": &dog.portrait_id,
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339(),
        };
        let res = self
            .db
            .collection::<Document>("dogs")
            .insert_one(dog, None)
            .await
            .map_err(|e| Error::new("failed to create dog").with_cause(e))?;
        res.inserted_id
            .as_object_id()
            .ok_or(Error::new("failed to create dog").with_cause("invalid inserted id"))
            .map(|id| id.to_string())
    }

    async fn delete_breed(&self, id: &str) -> Result<bool, Error> {
        self.db
            .collection::<Breed>("breeds")
            .delete_one(doc! {"_id": ObjectId::parse_str(id).map_err(|e| Error::new("failed to delete breed").with_cause(e))?}, None)
            .await
            .map_err(|e| Error::new("failed to delete breed").with_cause(e))
            .map(|res| res.deleted_count > 0)
    }

    async fn delete_dog(&self, id: &str) -> Result<bool, Error> {
        self.db
            .collection::<Breed>("dogs")
            .delete_one(doc! {"_id": ObjectId::parse_str(id).map_err(|e| Error::new("failed to delete dog").with_cause(e))?}, None)
            .await
            .map_err(|e| Error::new("failed to delete dog").with_cause(e))
            .map(|res| res.deleted_count > 0)
    }

    async fn update_dog(&self, id: &str, dog: &DogUpdate) -> Result<bool, Error> {
        let mut update = doc! {};
        if let Some(name) = &dog.name {
            update.insert("name", name);
        }
        if let Some(gender) = &dog.gender {
            update.insert("gender", gender);
        }
        if let Some(breed) = &dog.breed {
            update.insert("breed", &breed.id);
        }
        if let Some(birthday) = &dog.birthday {
            update.insert("birthday", birthday);
        }
        if let Some(is_sterilized) = &dog.is_sterilized {
            update.insert("is_sterilized", is_sterilized);
        }
        if let Some(introduction) = &dog.introduction {
            update.insert("introduction", introduction);
        }
        if let Some(owner_id) = &dog.owner_id {
            update.insert("owner_id", owner_id);
        }
        if let Some(tags) = &dog.tags {
            update.insert("tags", tags);
        }
        if let Some(portrait_id) = &dog.portrait_id {
            update.insert("portrait_id", portrait_id);
        }
        if !update.is_empty() {
            update.insert("updated_at", Local::now().to_rfc3339());
        }
        Ok(self
            .db
            .collection::<DogUpdate>("dogs")
            .update_one(
                doc! {
                    "_id": ObjectId::parse_str(id).map_err(|e| Error::new("failed to update dog").with_cause(e))?
                },
                doc! { "$set": update},
                None,
            )
            .await
            .map_err(|e| Error::new("failed to update dog").with_cause(e))?
            .modified_count
            > 0)
    }

    async fn query_breeds(&self, query: &BreedQuery) -> Result<(Vec<Breed>, i64), Error> {
        let mut q = doc! {};
        if let Some(category) = &query.category {
            q.insert("category", category.to_string());
        }
        let count = self
            .db
            .collection::<Breed>("breeds")
            .count_documents(q.clone(), None)
            .await
            .map_err(|e| Error::new("failed to query breeds").with_cause(e))?;
        let breeds = self
            .db
            .collection::<Breed>("breeds")
            .find(
                q,
                FindOptions::builder()
                    .projection(doc! {
                        "id": { "$toString": "$_id" },
                        "category": 1,
                        "name": 1,
                        "created_at": 1,
                        "updated_at": 1,
                    })
                    .build(),
            )
            .await
            .map_err(|e| Error::new("failed to query breeds").with_cause(e))?
            .try_collect::<Vec<Breed>>()
            .await
            .map_err(|e| Error::new("failed to query breeds").with_cause(e))?;
        Ok((breeds, count as i64))
    }

    async fn query_dogs(&self, query: &DogQuery) -> Result<(Vec<Dog>, i64), Error> {
        let mut q = doc! {};
        if let Some(owner_id) = &query.owner_id {
            q.insert("owner_id", owner_id);
        }
        if let Some(id_in) = &query.id_in {
            q.insert(
                "_id",
                doc! { "$in": id_in.deref().iter().map(|id| ObjectId::parse_str(id).map_err(|e| Error::new("failed to query my dogs").with_cause(e))).collect::<Result<Vec<_>, Error>>()? },
            );
        }
        let count = self
            .db
            .collection::<Dog>("dogs")
            .count_documents(q.clone(), None)
            .await
            .map_err(|e| Error::new("failed to query my dogs").with_cause(e))?;
        let dogs = self
            .db
            .collection::<Dog>("dogs")
            .aggregate(
                vec![
                    doc! {
                        "$match": q,
                    },
                    doc! {
                        "$limit": query.pagination.as_ref().map(|p| p.limit)
                    },
                    doc! {
                        "$skip": query.pagination.as_ref().map(|p| p.skip)
                    },
                    doc! {
                        "$addFields": {
                            "breed_id": { "$toObjectId": "$breed" }
                        }
                    },
                    doc! {
                        "$lookup": {
                            "from": "breeds",
                            "localField": "breed_id",
                            "foreignField": "_id",
                            "as": "breed",
                            "pipeline": [
                                {
                                    "$project": {
                                        "id": { "$toString": "$_id" },
                                        "category": 1,
                                        "name": 1,
                                        "created_at": 1,
                                        "updated_at": 1,
                                    }

                                }
                            ]

                        }
                    },
                    doc! {
                        "$project": {
                            "id": { "$toString": "$_id" },
                            "name": 1,
                            "gender": 1,
                            "breed": { "$arrayElemAt": [ "$breed", 0 ] } ,
                            "birthday": 1,
                            "is_sterilized": 1,
                            "introduction": 1,
                            "owner_id": 1,
                            "tags": 1,
                            "portrait_id": 1,
                            "created_at": 1,
                            "updated_at": 1,
                        }
                    },
                ],
                None,
            )
            .await
            .map_err(|e| Error::new("failed to query my dogs").with_cause(e))?
            .try_collect::<Vec<Document>>()
            .await
            .map(|ds| ds.into_iter().map(|d| from_document::<Dog>(d).unwrap()).collect())
            .map_err(|e| Error::new("failed to query my dogs").with_cause(e))?;
        Ok((dogs, count as i64))
    }

    async fn exists_dog(&self, query: &DogQuery) -> Result<bool, Error> {
        let mut q = doc! {};
        if let Some(id) = &query.id {
            q.insert("_id", ObjectId::parse_str(id).map_err(|e| Error::new("failed to query my dogs").with_cause(e))?);
        }
        if let Some(owner_id) = &query.owner_id {
            q.insert("owner_id", owner_id);
        }
        Ok(self
            .db
            .collection::<Dog>("dogs")
            .count_documents(q.clone(), None)
            .await
            .map_err(|e| Error::new("failed to query my dogs").with_cause(e))?
            > 0)
    }
}

// #[cfg(test)]
// mod test {

//     use super::*;
//     use mongodb::Client;

//     #[tokio::test]
//     async fn test_create_breed() {
//         let client = Client::with_uri_str("mongodb://localhost:27017").await.expect("Failed to connect to MongoDB");
//         let db = client.database("test");
//         let repo = MongoDB::new(db);
//         let id = repo.create_breed(BreedCreate { name: "金毛".to_owned() }).await.expect("Failed to create breed");
//         println!("{}", id);
//     }

//     #[tokio::test]
//     async fn delete_breeds() {
//         let client = Client::with_uri_str("mongodb://localhost:27017").await.expect("Failed to connect to MongoDB");
//         let db = client.database("test");
//         let repo = MongoDB::new(db);
//         let id = repo.create_breed(BreedCreate { name: "金毛".to_owned() }).await.expect("Failed to create breed");
//         repo.create_breed(BreedCreate { name: "拉布拉多".to_owned() }).await.expect("Failed to create breed");
//         let deleted = repo.delete_breeds(BreedQuery { id_eq: Some(id) }).await.expect("Failed to delete breeds");
//         assert!(deleted == 1);
//         repo.delete_breeds(BreedQuery { id_eq: None }).await.expect("Failed to delete breeds");
//     }

//     #[tokio::test]
//     async fn query_breeds() {
//         let client = Client::with_uri_str("mongodb://localhost:27017").await.expect("Failed to connect to MongoDB");
//         let db = client.database("test");
//         let repo = MongoDB::new(db);
//         repo.create_breed(BreedCreate { name: "金毛".to_owned() }).await.expect("Failed to create breed");
//         repo.create_breed(BreedCreate { name: "拉布拉多".to_owned() }).await.expect("Failed to create breed");
//         let (breeds, total) = repo
//             .query_breeds(BreedQuery { id_eq: None }, Some(Pagination { page: 1, size: 1 }))
//             .await
//             .expect("Failed to query breeds");
//         println!("breeds: {:?}, total: {}", breeds, total);
//     }
// }
