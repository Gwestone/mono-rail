use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}
