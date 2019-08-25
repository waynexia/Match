#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_derive;

extern crate rand;

use actix_web::web::{get, post, resource, JsonConfig};
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod api;
mod infos;
mod models;
mod schema;
mod utils;

fn main() -> std::io::Result<()> {
    utils::establish_connection();

    let manager = ConnectionManager::<SqliteConnection>::new("./sql/local-db/match_base.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(api::ping)
            //.service(add_game)
            .service(
                resource("/api/sign_up")
                    .data(JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(post().to_async(api::add_user)),
            )
            .service(
                resource("/api/spider/add_game")
                    .data(web::JsonConfig::default().limit(1024))
                    .route(post().to_async(api::add_game)),
            )
            .service(resource("/api/random_game").route(get().to_async(api::random_game)))
            .service(resource("/api/ping_post").route(post().to_async(api::ping_post)))
            .service(
                resource("/api/get_detail")
                    .data(web::JsonConfig::default().limit(1024))
                    .route(post().to_async(api::get_detail)),
            )
            .service(
                resource("/api/add_wishlist")
                    .data(web::JsonConfig::default())
                    .route(post().to_async(api::add_wishlist)),
            )
            .service(
                resource("/api/login")
                .data(web::JsonConfig::default())
                .route(post().to_async(api::login))
            )
            .service(
                resource("/api/game_list")
                .route(get().to_async(api::get_game_list))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
