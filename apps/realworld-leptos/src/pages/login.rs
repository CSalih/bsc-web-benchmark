use crate::auth::LoginUser;
use crate::models::User;
use crate::utils;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginRequest {
    pub user: LoginCommand,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginResponse {
    pub user: LoginUser,
}

#[component]
pub fn Login(
    set_user: WriteSignal<Option<User>>,
    access_token: ReadSignal<Option<String>>,
    set_access_token: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (error_message, set_error_message) = signal(String::from(""));
    let (username, set_username) = signal(String::from(""));
    let (password, set_password) = signal(String::from(""));

    Effect::new(move || {
        if access_token.with(Option::is_some) {
            let navigate = hooks::use_navigate();
            navigate("/", NavigateOptions::default());
        }
    });

    view! {
        <Title text="Login" />
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">"Login"</h1>

                        <p class="error-messages text-xs-center">{error_message}</p>
                        <form on:submit=move |e| {
                            e.prevent_default();
                            spawn_local(async move {
                                let login_command = LoginCommand {
                                    email: username.get_untracked(),
                                    password: password.get_untracked(),
                                };
                                login(&login_command, set_user, set_access_token, set_error_message)
                                    .await;
                            });
                        }>
                            <fieldset class="form-group">
                                <input
                                    name="username"
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Your Username"
                                    bind:value=(username, set_username)
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    name="password"
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    bind:value=(password, set_password)
                                />
                            </fieldset>
                            <a href="/reset_password">Reset password</a>
                            <button class="btn btn-lg btn-primary pull-xs-right">"Sign in"</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn login(
    login_command: &LoginCommand,
    set_user: WriteSignal<Option<User>>,
    set_access_token: WriteSignal<Option<String>>,
    set_error_message: WriteSignal<String>,
) {
    let url = "http://localhost:8080/api/users/login";
    let client = reqwest::Client::new();
    let login_request = LoginRequest {
        user: login_command.clone(),
    };
    let res = client.post(url).json(&login_request).send().await;
    match utils::response_to_value(res).await {
        Ok(val) => {
            let login_res = serde_json::from_value::<LoginResponse>(val).unwrap();
            set_access_token.set(Some(login_res.user.token.clone()));
            set_user.set(Some(login_res.user.into()));
        }
        Err(err) => {
            set_error_message.set(err);
        }
    };
}
