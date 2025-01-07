use leptos::prelude::*;

mod api;

pub use api::*;

pub type LogoutAction = Action<(), bool>;
pub type LoginAction = Action<LoginCommand, Result<LoginMessages, ServerFnError>>;
pub type SignupSignal = Action<SignupCommand, Result<SignupResponse, ServerFnError>>;


#[derive(Clone, Debug)]
pub struct AuthContext {
    pub is_authenticated: ReadSignal<bool>,
    pub username: ReadSignal<Option<String>>,
}

impl AuthContext {
    pub(crate) fn new(is_authenticated: ReadSignal<bool>, username: ReadSignal<Option<String>>) -> Self {
        Self {
            is_authenticated,
            username,
        }
    }
}