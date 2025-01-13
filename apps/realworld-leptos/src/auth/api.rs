use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignupResponse {
    ValidationError(String),
    CreateUserError(String),
    Success,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginMessages {
    Successful,
    Unsuccessful,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn validate_signup(signup_command: &SignupCommand) -> Result<crate::models::User, String> {
    crate::models::User::default()
        .set_username(signup_command.username.clone())?
        .set_password(signup_command.password.clone())?
        .set_email(signup_command.email.clone())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub user: LoginUser,
}

impl Into<crate::models::User> for LoginUser {
    fn into(self) -> crate::models::User {
        crate::models::User::new(self.username, self.email)
            .set_bio(self.bio.unwrap_or_default()).unwrap()
            .set_image(self.image.unwrap_or_default()).unwrap()
    }
}
