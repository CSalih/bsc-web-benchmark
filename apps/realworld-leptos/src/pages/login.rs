use crate::auth::{LoginAction, LoginCommand, LoginMessages};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use log::error;

#[component]
pub fn Login(login: LoginAction) -> impl IntoView {
    let (username, set_username) = signal(String::from(""));
    let (password, set_password) = signal(String::from(""));
    let result_of_call = login.value();

    let error = move || {
        result_of_call.with(|msg| {
            msg.as_ref()
                .map(|inner| match inner {
                    Ok(LoginMessages::Unsuccessful) => "Incorrect user or password",
                    Ok(LoginMessages::Successful) => {
                        let navigate = hooks::use_navigate();
                        navigate("/", NavigateOptions::default());

                        "Done"
                    }
                    Err(x) => {
                        error!("Problem during login: {x:?}");
                        "There was a problem, try again later"
                    }
                })
                .unwrap_or_default()
        })
    };

    view! {
        <Title text="Login" />
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">"Login"</h1>

                        <p class="error-messages text-xs-center">{error}</p>
                        <form on:submit=move |e| {
                            e.prevent_default();
                            login
                                .dispatch(LoginCommand {
                                    username: username.get_untracked(),
                                    password: password.get_untracked(),
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
