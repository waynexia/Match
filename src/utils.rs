use crate::models::User;
use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    // dotenv.ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "mysql://localhost/test";
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connect to {}", database_url))
}

pub fn create_user(conn: &MysqlConnection, nickname: &str, email: &str, password: &str) -> User {
    use crate::schema::user;
    use crate::models::NewUser;

    let new_user = NewUser{
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
