#[derive(Deserialize, Debug, Serialize)]
pub struct UserInfo {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct get_detail_info {
    pub gamename: String,
}
