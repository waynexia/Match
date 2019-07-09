use super::schema::user;

#[derive(Queryable)]
pub struct User {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub nickname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
