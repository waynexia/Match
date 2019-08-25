use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use super::infos;
use super::models;
use super::schema;
use super::utils;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn add_user(
    pool: web::Data<Pool>,
    info: Json<infos::UserInfo>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", info);
    let conn: &SqliteConnection = &pool.get().unwrap();
    utils::create_user(conn, &info.nickname, &info.email, &info.password);
    HttpResponse::Ok().json(info.0)
}

/*
    todo: add unique check before insert
*/
pub fn add_game(
    pool: web::Data<Pool>,
    this_game_json: web::Json<models::game_infos>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::game::dsl::*;
    println!("request: {:?}", req);

    let conn: &SqliteConnection = &pool.get().unwrap();

    // let this_game_obj= serde_json::from_str::<GameInfo>(&this_game_json).unwrap();
    let this_game_obj = &this_game_json.into_inner();
    println!("model: {:?}", this_game_obj);

    // let result = game
    //     .filter(gamename.eq(&this_game_json.gamename))
    //     .load::<models::game_infos>(&conn)
    //     .expect("Errrrrrror");
    // println!("query: {:?}", result.len());

    diesel::insert_into(game)
        .values(this_game_obj)
        .execute(conn)
        .unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}

pub fn get_detail(
    pool: web::Data<Pool>,
    requested_json: web::Json<infos::GetDetailInfo>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::game::dsl::*;

    let conn = utils::establish_connection();

    let result = game
        .filter(gamename.eq(&requested_json.gamename))
        .load::<models::game_infos>(&conn)
        .expect("Errorr");

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(result)
}
pub fn get_game_list(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    use schema::game::dsl::*;

    let conn = utils::establish_connection();

    let all_games = game.load::<models::game_infos>(&conn).expect("error");

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(all_games)
}

pub fn random_game(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    use rand::Rng;
    use schema::game::dsl::*;
    let mut rng = rand::thread_rng();

    let conn = utils::establish_connection();
    let results = game.load::<models::game_infos>(&conn).expect("error");
    let len = results.len();
    let index: usize = rng.gen_range(0, len);
    let result = &results[index];

    println!("{:?}", result);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .json(result)
}

pub fn add_wishlist(
    pool: web::Data<Pool>,
    new_wl_json: web::Json<models::wishlist_infos>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::wishlist::dsl::*;

    let conn: &SqliteConnection = &pool.get().unwrap();

    if !utils::is_user_exist(conn, new_wl_json.nickname.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body("user not exist");
    }

    if !utils::is_game_exist(conn, new_wl_json.gamename.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body("game not exist");
    }

    if utils::is_wishlist_exist(conn, new_wl_json.nickname, new_wl_json.gamename: &str) {
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body("wishlist already exist");
    }

    let new_wishlist_item = new_wl_json.into_inner();

    diesel::insert_into(wishlist)
        .values(new_wishlist_item)
        .execute(conn)
        .unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}

// pub fn del_wishlist(
//     pool: web::Data<Pool>,
//     wl_json: web::Json<models::wishlist_infos>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     use schema::wishlist::dsl::*;

//     let conn: &SqliteConnection = &pool.get().unwrap();

//     if !utils::is_wishlist_exist(conn, wl_json.nickname, wl_json.gamename: &str) {
//     return HttpResponse::build(StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body("wishlist not exist");
//     }


// }

#[get("/api/ping")]
pub fn ping(sess: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("pong");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("pong"))
}

pub fn ping_post(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}
