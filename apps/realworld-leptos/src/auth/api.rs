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
    pub username: String,
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
pub struct LoginResponse {
    pub user: crate::models::User,
}

pub async fn login_user(login_command: &LoginCommand) -> LoginResponse {
    let url = "http://localhost:8080/api/users/login";

    let mut json_body = std::collections::HashMap::new();
    json_body.insert("user", login_command);


    reqwest::Client::new()
        .post(url)
        .json(&json_body)
        .send()
        .await
        .unwrap()
        .json::<LoginResponse>()
        .await
        .unwrap()
}

