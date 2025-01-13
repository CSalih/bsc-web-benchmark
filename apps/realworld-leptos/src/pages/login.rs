use crate::auth::{LoginCommand, LoginResponse};
use crate::models::User;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;
use leptos_router::*;
use serde_json::Value;

#[component]
pub fn Login(
    set_user: WriteSignal<Option<User>>,
    access_token: ReadSignal<Option<String>>,
    set_access_token: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (error_message, set_error_message) = signal(String::from(""));
    let (username, set_username) = signal(String::from(""));
    let (password, set_password) = signal(String::from(""));

    let login = move |email: String, password: String| async move {
        let url = "http://localhost:8080/api/users/login";
        let mut json_body = std::collections::HashMap::new();
        let login_command = LoginCommand { email, password };
        json_body.insert("user", login_command);

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

        let login_res = res
            .bytes()
            .await
            .ok()
            .map(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
            .flatten();

        match login_res {
            Some(login_res) => {
                let login_res = serde_json::from_value::<LoginResponse>(login_res).unwrap();
                set_access_token.set(Some(login_res.user.token.clone()));
                set_user.set(Some(login_res.user.into()));

            }
            None => {
                // Response is not valid json
                set_error_message.set("unknown error".to_string());
            }
        }
    };

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
                                login(username.get_untracked(), password.get_untracked()).await;
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
