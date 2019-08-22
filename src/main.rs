#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_derive;

use actix_session::Session;
use actix_web::http::{Method, StatusCode};
use actix_web::web::{post, resource, Json, JsonConfig};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod schema;
mod utils;

use self::models::*;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Deserialize, Debug, Serialize)]
struct UserInfo {
    nickname: String,
    email: String,
    password: String,
}

// #[derive(Deserialize, Debug, Serialize,Insertable)]
// struct GameInfo {
//     <models::game_infos as Trait>::gamename: String,
//     <models::game_infos as Trait>::price: String,
//     <models::game_infos as Trait>::link: String,
//     <models::game_infos as Trait>::image_url: String,
//     <models::game_infos as Trait>::desc: String,
// }

fn add_user(pool: web::Data<Pool>, info: Json<UserInfo>, req: HttpRequest) -> HttpResponse {
    println!("{:?}", info);
    let conn: &SqliteConnection = &pool.get().unwrap();
    utils::create_user(conn, &info.nickname, &info.email, &info.password);
    HttpResponse::Ok().json(info.0)
}

fn add_game(
    pool: web::Data<Pool>,
    this_game_json: web::Json<game_infos>,
    req: HttpRequest,
) -> HttpResponse {
    use self::models::*;
    use self::schema::game::dsl::*;
    println!("request: {:?}", req);

    let conn = utils::establish_connection();

    // let this_game_obj= serde_json::from_str::<GameInfo>(&this_game_json).unwrap();
    let this_game_obj= &this_game_json.into_inner();
    println!("model: {:?}", this_game_obj);

    // let result = game
    //     .filter(gamename.eq(&this_game_json.gamename))
    //     .load::<game_infos>(&conn)
    //     .expect("Errrrrrror");
    // println!("query: {:?}", result.len());

    diesel::insert_into(game)
    .values(this_game_obj)
    .execute(&conn)
    .unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}

#[get("/api/ping")]
fn ping(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("pong");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("pong"))
}

fn main() -> std::io::Result<()> {
    utils::establish_connection();

    let manager = ConnectionManager::<SqliteConnection>::new("./sql/local-db/test_sqlite_db.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(ping)
            //.service(add_game)
            .service(
                resource("/api/add_user")
                    .data(JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(post().to_async(add_user)),
            )
            .service(
                resource("/api/spider/add_game")
                    .data(web::JsonConfig::default().limit(1024))
                    .route(post().to_async(add_game)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
