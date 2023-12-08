use actix_web::web;

#[macro_use] extern crate actix_web;

mod api;
mod controllers;
mod dao;
mod requests;
mod responses;
mod services;

#[actix::main] 
async fn main() -> Result<(), std::io::Error> {
    nightmare_common::app::serve(|| {
        web::scope("")
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
    }).await
}
