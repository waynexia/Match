use crate::models::User;
use diesel::prelude::*;

use super::models;
use super::schema;

pub fn establish_connection() -> SqliteConnection {
    // dotenv.ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "./sql/local-db/match_base.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connect to {}", database_url))
}

pub fn create_user(conn: &SqliteConnection, nickname: &str, email: &str, password: &str) -> User {
    use crate::models::NewUser;
    use crate::schema::user;

    let new_user = NewUser {
        nickname,
        email,
        password,
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error when saving new user");

    user::table.order(user::nickname).first(conn).unwrap()
}

pub fn is_user_exist(conn: &SqliteConnection, nickname_arg: &str) -> bool {
    use schema::user::dsl::*;

    let result = user
        .filter(nickname.eq(nickname_arg))
        .load::<models::User>(conn)
        .expect("Error while quering user");

    result.len() != 0
}

pub fn is_game_exist(conn: &SqliteConnection, gamename_arg: &str) -> bool {
    use schema::game::dsl::*;

    let result = game
        .filter(gamename.eq(gamename_arg))
        .load::<models::game_infos>(conn)
        .expect("Error while quering game");

    result.len() != 0
}

pub fn is_wishlist_exist(conn: &SqliteConnection, nickname_arg: &str, gamename_arg: &str) -> bool {
    use schema::wishlist::dsl::*;

    let result = wishlist
        .filter(nickname.eq(nickname_arg))
        .filter(gamename.eq(gamename_arg))
        .load::<models::wishlist_infos>(conn)
        .expect("Error while quering wishlist");

    result.len() != 0
}
