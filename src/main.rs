#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_derive;

extern crate rand;

use actix_session::Session;
use actix_web::http::{Method, StatusCode};
use actix_web::web::{get, post, resource, Json, JsonConfig};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod api;
mod models;
mod schema;
mod utils;
mod infos;

use self::models::*;
use self::utils::*;
use self::infos::*;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// #[derive(Deserialize, Debug, Serialize,Insertable)]
// struct GameInfo {
//     <models::game_infos as Trait>::gamename: String,
//     <models::game_infos as Trait>::price: String,
//     <models::game_infos as Trait>::link: String,
//     <models::game_infos as Trait>::image_url: String,
//     <models::game_infos as Trait>::desc: String,
// }

fn main() -> std::io::Result<()> {
    utils::establish_connection();

    let manager = ConnectionManager::<SqliteConnection>::new("./sql/local-db/test_sqlite_db.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(api::ping)
            //.service(add_game)
            .service(
                resource("/api/add_user")
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
    })
    .bind("127.0.0.1:8080")?
    .run()
}
