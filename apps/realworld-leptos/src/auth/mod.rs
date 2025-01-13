use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub type LogoutAction = Action<(), bool>;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user: ReadSignal<Option<crate::models::User>>,
    pub access_token: ReadSignal<Option<String>>,
}

impl AuthContext {
    pub(crate) fn new(
        user: ReadSignal<Option<crate::models::User>>,
        access_token: ReadSignal<Option<String>>,
    ) -> Self {
        Self { user, access_token }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl Into<crate::models::User> for LoginUser {
    fn into(self) -> crate::models::User {
        crate::models::User::new(self.username, self.email)
            .set_bio(self.bio.unwrap_or_default())
            .unwrap()
            .set_image(self.image.unwrap_or_default())
            .unwrap()
    }
}
