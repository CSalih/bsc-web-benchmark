use crate::auth::LoginUser;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  SignupResponse {
    pub user: LoginUser,
}

pub fn validate_signup(signup_command: &SignupCommand) -> Result<User, String> {
    User::default()
        .set_username(signup_command.username.clone())?
        .set_password(signup_command.password.clone())?
        .set_email(signup_command.email.clone())
}

#[component]
pub fn SignupPage() -> impl IntoView {
    let (signup_success, set_signup_success) = signal(false);
    let (error_message, set_error_message) = signal(String::from(""));
    let (email, set_email) = signal(String::from(""));
    let (username, set_username) = signal(String::from(""));
    let (password, set_password) = signal(String::from(""));

    let signup = move |username: String, email: String, password: String| async move {
        let signup_command = SignupCommand { username, email, password };
        if let Err(err) = validate_signup(&signup_command) {
            set_error_message.set(err);
            return;
        }

        let url = "http://localhost:8080/api/users";
        let mut json_body = std::collections::HashMap::new();
        json_body.insert("user", signup_command);

        let client = reqwest::Client::new();
        let res = client.post(url).json(&json_body).send().await;

        let Ok(res) = res else {
            set_error_message.set("unknown error".to_string());
            return;
        };
        if !res.status().is_success() {
            let error_message = res
                .json::<Value>()
                .await
                .ok()
                .and_then(|json| json.get("error").and_then(|msg| msg.as_str().map(String::from)))
                .unwrap_or_else(|| "Unknown error".to_string());
            set_error_message.set(error_message);
            return;
        }

        let signup_res = res
            .bytes()
            .await
            .ok()
            .map(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
            .flatten();

        match signup_res {
            Some(login_res) => {
                let _ = serde_json::from_value::<SignupResponse>(login_res).unwrap();
                set_signup_success.set(true);
            }
            None => {
                // Response is not valid json
                set_error_message.set("unknown error".to_string());
            }
        }
    };

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
                                signup(
                                    username.get_untracked(),
                                    email.get_untracked(),
                                    password.get_untracked()
                                ).await;
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
