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

    if utils::is_user_exist(conn, info.nickname.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("user already exist");
    }

    let salt = utils::gen_salt();
    let password = utils::hash(&(info.password.to_owned() + &salt));

    utils::create_user(conn, &info.nickname, &info.email, &password, &salt);
    HttpResponse::Ok().json(info.0)
}

pub fn login(
    pool: web::Data<Pool>,
    info: Json<infos::LoginInfo>,
    req: HttpRequest,
) -> HttpResponse {
    let conn: &SqliteConnection = &pool.get().unwrap();

    if !utils::is_user_exist(conn, info.nickname.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("username or password not correct!");
    }

    use schema::user::dsl::*;

    let result = user
        .filter(nickname.eq(info.nickname.clone()))
        .load::<models::User>(conn)
        .expect("Error while quering user");

    println!("{:?}", result);

    if utils::check_password(&result[0].password, &info.password, &result[0].salt) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("success login");
    } else {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("username or password not correct");
    }
}

pub fn add_game(
    pool: web::Data<Pool>,
    this_game_json: web::Json<models::game_infos>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::game::dsl::*;
    println!("request: {:?}", req);

    let conn: &SqliteConnection = &pool.get().unwrap();

    if utils::is_game_exist(conn, this_game_json.gamename.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("game already exist");
    }

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
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}

pub fn get_detail(
    pool: web::Data<Pool>,
    requested_json: web::Json<infos::GetDetailInfo>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::game::dsl::*;

    let conn: &SqliteConnection = &pool.get().unwrap();

    if !utils::is_game_exist(conn, requested_json.gamename.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("game not exist");
    }

    let result = game
        .filter(gamename.eq(&requested_json.gamename))
        .load::<models::game_infos>(conn)
        .expect("Errorr");

    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("application/json; charset=utf-8")
        .json(result)
}
pub fn get_game_list(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    use schema::game::dsl::*;

    let conn = utils::establish_connection();

    let all_games = game.load::<models::game_infos>(&conn).expect("error");

    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
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
    if len == 0 {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("no game here");
    }

    let index: usize = rng.gen_range(0, len);
    let result = &results[index];

    println!("{:?}", result);

    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
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
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("user not exist");
    }

    if !utils::is_game_exist(conn, new_wl_json.gamename.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("game not exist");
    }

    if utils::is_wishlist_exist(
        conn,
        new_wl_json.nickname.as_str(),
        new_wl_json.gamename.as_str(),
    ) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("wishlist already exist");
    }

    let new_wishlist_item = new_wl_json.into_inner();

    diesel::insert_into(wishlist)
        .values(new_wishlist_item)
        .execute(conn)
        .unwrap();

    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}

pub fn del_wishlist(
    pool: web::Data<Pool>,
    wl_json: web::Json<models::wishlist_infos>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::wishlist::dsl::*;

    let conn: &SqliteConnection = &pool.get().unwrap();

    if !utils::is_wishlist_exist(conn, wl_json.nickname.as_str(), wl_json.gamename.as_str()) {
        return HttpResponse::build(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "content-type")
            .header("Access-Control-Request-Method", "GET,POST")
            .content_type("text/html; charset=utf-8")
            .body("wishlist not exist");
    }

    diesel::delete(
        wishlist
            .filter(nickname.eq(wl_json.nickname.as_str()))
            .filter(gamename.eq(wl_json.gamename.as_str())),
    )
    .execute(conn)
    .unwrap();

    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}

pub fn get_wishlist(
    pool: web::Data<Pool>,
    info: web::Json<infos::GetWishlistInfo>,
    req: HttpRequest,
) -> HttpResponse {
    use schema::wishlist::dsl::*;
    let conn: &SqliteConnection = &pool.get().unwrap();

    let result = wishlist
        .filter(nickname.eq(info.nickname.clone()))
        .load::<models::wishlist_infos>(conn)
        .expect("Error while quering user");

    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("text/html; charset=utf-8")
        .json(result)
}

#[get("/api/ping")]
pub fn ping(sess: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("pong");
    Ok(HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("text/html; charset=utf-8")
        .body("pong"))
}

pub fn ping_post(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    HttpResponse::build(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "content-type")
        .header("Access-Control-Request-Method", "GET,POST")
        .content_type("text/html; charset=utf-8")
        .body("OKKKKKKK")
}
