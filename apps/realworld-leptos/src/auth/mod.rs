use leptos::prelude::*;

mod api;

pub use api::*;

pub type LogoutAction = Action<(), bool>;
pub type SignupSignal = Action<SignupCommand, Result<SignupResponse, ServerFnError>>;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user: ReadSignal<Option<crate::models::User>>,
    pub access_token: ReadSignal<Option<String>>,
}

impl AuthContext {
    pub(crate) fn new(
        user: ReadSignal<Option<crate::models::User>>,
        access_token: ReadSignal<Option<String>>
    ) -> Self {
        Self { user, access_token }
    }
}
