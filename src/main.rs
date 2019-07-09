#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_derive;

use actix_session::Session;
use actix_web::http::{Method, StatusCode};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web::web::{Json,resource,JsonConfig,post};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod schema;
mod utils;

#[derive(Deserialize,Debug,Serialize)]
struct UserInfo{
    nickname: String,
    email: String,
    password: String,
}

fn add_user(info: Json<UserInfo>,req: HttpRequest) -> HttpResponse{
    println!("{:?}",info);
    HttpResponse::Ok().json(info.0)
}

#[post("/api/spider/add_game")]
fn add_game(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(""))
}

#[get("/api/ping")]
fn ping(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("pong");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(""))
}

fn main() -> std::io::Result<()> {
    let conn = utils::establish_connection();
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(add_game)
            .service(
                resource("/api/add_user")
                    .data(JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(post().to_async(add_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
