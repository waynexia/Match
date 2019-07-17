use crate::models::User;
use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    // dotenv.ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "./sql/local-db/test_sqlite_db.db";
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
