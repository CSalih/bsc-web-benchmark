use crate::auth::LoginUser;
use crate::models::User;
use crate::utils;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignupCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignupRequest {
    pub user: SignupCommand,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignupResponse {
    pub user: LoginUser,
}

#[component]
pub fn SignupPage() -> impl IntoView {
    let (signup_success, set_signup_success) = signal(false);
    let (error_message, set_error_message) = signal(String::from(""));
    let (email, set_email) = signal(String::from(""));
    let (username, set_username) = signal(String::from(""));
    let (password, set_password) = signal(String::from(""));

    // let signup = signup(set_signup_success, set_error_message);

    Effect::new(move || {
        if signup_success.get() {
            let navigate = hooks::use_navigate();
            navigate("/login", NavigateOptions::default());
        }
    });

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">"Sign up"</h1>
                        <p class="text-xs-center">
                            <a href="/login">"Have an account?"</a>
                        </p>

                        <p class="error-messages text-xs-center">{error_message}</p>

                        <form on:submit=move |e| {
                            e.prevent_default();
                            spawn_local(async move {
                                let signup_command = SignupCommand {
                                    username: username.get_untracked(),
                                    email: email.get_untracked(),
                                    password: password.get_untracked(),
                                };
                                signup(&signup_command, set_signup_success, set_error_message)
                                    .await;
                            });
                        }>
                            <fieldset class="form-group">
                                <input
                                    name="username"
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Your Username"
                                    required=true
                                    bind:value=(username, set_username)
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    name="email"
                                    class="form-control form-control-lg"
                                    type="email"
                                    placeholder="Email"
                                    required=true
                                    bind:value=(email, set_email)
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    name="password"
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    required=true
                                    bind:value=(password, set_password)
                                />
                            </fieldset>
                            <button type="submit" class="btn btn-lg btn-primary pull-xs-right">
                                "Sign up"
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn validate_signup(signup_command: &SignupCommand) -> Result<User, String> {
    User::default()
        .set_username(signup_command.username.clone())?
        .set_password(signup_command.password.clone())?
        .set_email(signup_command.email.clone())
}

async fn signup(
    signup_command: &SignupCommand,
    set_signup_success: WriteSignal<bool>,
    set_error_message: WriteSignal<String>,
) {
    if let Err(err) = validate_signup(&signup_command) {
        set_error_message.set(err);
        return;
    }

    let url = "http://localhost:8080/api/users";
    let client = reqwest::Client::new();
    let signup_request = SignupRequest {
        user: signup_command.clone(),
    };
    let res = client.post(url).json(&signup_request).send().await;
    match utils::response_to_value(res).await {
        Ok(val) => {
            let _ = serde_json::from_value::<SignupResponse>(val).unwrap();
            set_signup_success.set(true);
        }
        Err(err) => {
            set_error_message.set(err.into());
        }
    };
}
