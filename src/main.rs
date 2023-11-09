#![feature(async_fn_in_trait)]

mod core;
mod handlers;
mod middlewares;
mod repositories;

use core::service::Service;

use actix_web::{
    middleware::Logger,
    web::{get, post, put, resource, scope, Data},
    App, HttpServer,
};
use env_logger::Env;
use middlewares::response_encoding::ResponseEncoding;
use mongodb::Client;
use nb_from_env::{FromEnv, FromEnvDerive};
use repositories::mongodb::MongoDB;

#[derive(FromEnvDerive)]
pub struct Config {
    listen_address: String,
    #[env_default("info")]
    log_level: String,
    #[env_default("%t %r %s %D")]
    log_format: String,
    mongodb_uri: String,
    mongodb_database_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env();
    env_logger::init_from_env(Env::default().default_filter_or(config.log_level));
    let client = Client::with_uri_str(config.mongodb_uri).await.expect("failed to connect to mongodb");
    let service = Data::new(Service::new(MongoDB::new(client.database(&config.mongodb_database_name))));
    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .wrap(ResponseEncoding)
            .wrap(Logger::new(config.log_format.as_str()))
            .service(resource("breeds").post(handlers::breed::create_breed::<MongoDB>).get(handlers::breed::breeds::<MongoDB>))
            .service(
                scope("dogs")
                    .route("", post().to(handlers::dog::create_dog::<MongoDB>))
                    .route("", get().to(handlers::dog::dogs::<MongoDB>))
                    .route("", put().to(handlers::dog::update_dog::<MongoDB>))
                    .route("mine", get().to(handlers::dog::my_dogs::<MongoDB>))
                    .route("exists", get().to(handlers::dog::is_owner_of_the_dog::<MongoDB>))
                    .route("portrait", put().to(handlers::dog::update_dog_portrait::<MongoDB>)),
            )
    })
    .bind(config.listen_address)?
    .run()
    .await
}
