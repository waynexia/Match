use super::schema::game;
use super::schema::user;
use super::schema::wishlist;

// table user

#[derive(Queryable,Debug)]
pub struct User {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub nickname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
}

// table game

#[derive(Queryable, Insertable, Deserialize, Debug, Serialize)]
#[table_name = "game"]
pub struct game_infos {
    pub gamename: String,
    pub oringinal_price: f32,
    pub current_price: f32,
    pub lowest_price: f32,
    pub link: String,
    pub image_url: String,
    pub desc: String,
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name = "wishlist"]
pub struct wishlist_infos {
    pub nickname: String,
    pub gamename: String,
    pub email_alert: bool,
}

// #[derive(Insertable)]
// #[table_name = "game"]
// pub struct GameInfo<'a> {
//     pub gamename: &'a str,
//     pub price: &'a f32,
//     pub link: &'a str,
//     pub image_url: &'a str,
//     pub desc: &'a str,
// }
