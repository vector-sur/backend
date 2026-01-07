use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub phone: i64,
    pub email: String,
    pub password: String,
}
