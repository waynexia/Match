#[macro_use]
extern crate actix_web;

use actix_session::{Session};
use actix_web::http::{Method, StatusCode};
use actix_web::{App, HttpResponse, HttpRequest,HttpServer,Result};

#[post("/api/spider/add_game")]
fn add_game(session: Session,req: HttpRequest) -> Result<HttpResponse>{
    println!("{:?}",req);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(""))
}

#[get("/api/ping")]
fn ping(session: Session,req:HttpRequest) -> Result<HttpResponse>{
    println!("pong");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(""))
}

fn main() ->std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(add_game)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
