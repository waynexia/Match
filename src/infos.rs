#[derive(Deserialize, Debug, Serialize)]
pub struct UserInfo {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetDetailInfo {
    pub gamename: String,
}

// #[derive(Deserialize, Serialize,Insertable)]
// pub struct WishlistInfo {
//     pub nickname: String,
//     pub gamename: String,
//     pub email_alert: bool,
// }
