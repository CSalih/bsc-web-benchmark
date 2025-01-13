use leptos::prelude::*;
use serde::{Deserialize, Serialize};



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
