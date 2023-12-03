#[macro_use] extern crate actix_web;

use std::{usize::MAX, env};

use actix_web::{HttpServer, App, web::{self, Data, PayloadConfig, JsonConfig, FormConfig}};
use nightmare_common::{database::{DB, self}, log};

mod api;
mod controllers;
mod dao;
mod requests;
mod responses;
mod services;

#[actix::main] 
async fn main() -> Result<(), std::io::Error> {
    let db = DB {
        auth: database::connect(env::var("AUTH_URL").unwrap()).await,
        main: database::connect(env::var("DATABASE_URL").unwrap()).await,
    };

    let auth = db.auth.ping().await;
    let main = db.main.ping().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::new(MAX))
            .app_data(JsonConfig::default().limit(MAX))
            .app_data(FormConfig::default().limit(MAX))
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(db.main.clone()))
            .service(api::service())
            .service(controllers::auth::login)
            .service(controllers::auth::authenticate)
            .service(controllers::auth::logout)
            .service(
                web::scope("/api/v1")
                    // user
                    .service(controllers::user::paginate)
                    .service(controllers::user::store)
                    .service(controllers::user::show)
                    .service(controllers::user::update_general_information)
                    .service(controllers::user::update_password)
                    .service(controllers::user::delete)
                    .service(controllers::user::sync_permissions)
                    .service(controllers::user::sync_roles)
                    // permission
                    .service(controllers::permission::paginate)
                    .service(controllers::permission::store)
                    .service(controllers::permission::show)
                    .service(controllers::permission::update)
                    .service(controllers::permission::delete)
                    // role
                    .service(controllers::role::paginate)
                    .service(controllers::role::store)
                    .service(controllers::role::show)
                    .service(controllers::role::update)
                    .service(controllers::role::delete)
            )
    })
        .workers(4)
        .bind(("0.0.0.0", 8000))?
        .run();

    log::info!(main, "Server started at http://localhost:8080",);
    log::info!(main, "Auth database status {:?}", auth);
    log::info!(main, "Auth database status {:?}", main);

    server.await
}
