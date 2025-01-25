use crate::auth::{LoginUser, LogoutAction};
use crate::models::User;
use crate::{auth, utils};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UpdateUserCommand {
    pub email: Option<String>,
    pub username: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UpdateUserRequest {
    pub user: UpdateUserCommand,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UpdateUserResponse {
    pub user: LoginUser,
}

#[component]
#[allow(non_snake_case)]
pub fn SettingsPage(logout: LogoutAction, update_user: WriteSignal<Option<User>>) -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let Some(user) = auth_context.user.get() else {
        panic!("ss");
    };

    let (error_messages, set_error_messages) = signal(Vec::<String>::new());
    let (profile_picture, set_profile_picture) = signal(user.image().unwrap_or_default());
    let (name, set_name) = signal(user.username());
    let (bio, set_bio) = signal(user.bio().unwrap_or_default());
    let (email, set_email) = signal(user.email());
    let (new_password, set_new_password) = signal(String::new());

    view! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Your Settings</h1>

                        <Show when=move || !error_messages.get().is_empty()>
                            <ul class="error-messages">
                                <For
                                    each=move || error_messages.get().into_iter().enumerate()
                                    key=|(index, _)| *index
                                    children=move |(_, message)| {
                                        let message = message.to_string();
                                        view! { <li>{message}</li> }
                                    }
                                />
                            </ul>
                        </Show>

                        <form on:submit=move |e| {
                            e.prevent_default();
                            spawn_local(async move {
                                let update_user_command = UpdateUserCommand {
                                    email: Some(email.get_untracked()),
                                    username: Some(name.get_untracked()),
                                    image: Some(profile_picture.get_untracked()),
                                    bio: Some(bio.get_untracked()),
                                    password: Some(new_password.get_untracked()),
                                };
                                login(
                                        &update_user_command,
                                        auth_context
                                            .access_token
                                            .get_untracked()
                                            .unwrap_or_default(),
                                        update_user,
                                        set_error_messages,
                                    )
                                    .await;
                            });
                        }>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="URL of profile picture"
                                        bind:value=(profile_picture, set_profile_picture)
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Your Name"
                                        bind:value=(name, set_name)
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control form-control-lg"
                                        rows="8"
                                        placeholder="Short bio about you"
                                        bind:value=(bio, set_bio)
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Email"
                                        bind:value=(email, set_email)
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="New Password"
                                        bind:value=(new_password, set_new_password)
                                    />
                                </fieldset>
                                <button class="btn btn-lg btn-primary pull-xs-right">
                                    Update Settings
                                </button>
                            </fieldset>
                        </form>
                        <hr />
                        <button
                            class="btn btn-outline-danger"
                            on:click=move |_| {
                                logout.dispatch(());
                            }
                        >
                            Or click here to logout.
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn login(
    update_user_command: &UpdateUserCommand,
    auth_token: String,
    set_user: WriteSignal<Option<User>>,
    set_error_messages: WriteSignal<Vec<String>>,
) {
    let url = "http://localhost:8080/api/user";
    let client = reqwest::Client::new();
    let login_request = UpdateUserRequest {
        user: update_user_command.clone(),
    };
    set_error_messages.set(Vec::new());
    let res = client
        .put(url)
        .header("Token", &auth_token.to_string())
        .json(&login_request)
        .send()
        .await;
    match utils::response_to_value(res).await {
        Ok(val) => {
            let login_res = serde_json::from_value::<UpdateUserResponse>(val).unwrap();
            set_user.set(Some(login_res.user.into()));
        }
        Err(err) => {
            set_error_messages.update(|errors| {
                errors.push(err.to_string());
            });
        }
    };
}
