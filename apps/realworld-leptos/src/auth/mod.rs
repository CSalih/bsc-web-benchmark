use leptos::prelude::*;

mod api;

pub use api::*;


pub type LogoutSignal = Action<LogoutAction, Result<(), ServerFnError>>;
pub type LoginSignal = Action<LoginAction, Result<LoginMessages, ServerFnError>>;
pub type SignupSignal = Action<SignupAction, Result<SignupResponse, ServerFnError>>;
pub type UsernameSignal = RwSignal<Option<String>>;
