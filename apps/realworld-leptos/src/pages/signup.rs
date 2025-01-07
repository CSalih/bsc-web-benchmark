use crate::auth::{SignupCommand, SignupResponse, SignupSignal};
use leptos::prelude::*;
use leptos_router::*;

#[component]
pub fn SignupPage(signup: SignupSignal) -> impl IntoView {
    let result_of_call = signup.value();

    let error = move || {
        result_of_call.with(|msg| {
            msg.as_ref()
                .map(|inner| match inner {
                    Ok(SignupResponse::Success) => {
                        let navigate = hooks::use_navigate();
                        navigate("/login", NavigateOptions::default());

                        "Done".into()
                    }
                    Ok(SignupResponse::ValidationError(x)) => {
                        format!("Problem while validating: {x}")
                    }
                    Ok(SignupResponse::CreateUserError(x)) => {
                        format!("Problem while creating user: {x}")
                    }
                    Err(_) => "There was a problem, try again later".into(),
                })
                .unwrap_or_default()
        })
    };

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">"Sign up"</h1>
                        <p class="text-xs-center">
                            <a href="/login">"Have an account?"</a>
                        </p>

                        <p class="error-messages text-xs-center">{error}</p>

                        <form on:submit=move |e| {
                            e.prevent_default();
                            if let Ok(data) = SignupCommand::from_event(&e) {
                                signup.dispatch(data);
                            }
                        }>
                            <fieldset class="form-group">
                                <input
                                    name="username"
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Your Username"
                                    required=true
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    name="email"
                                    class="form-control form-control-lg"
                                    type="email"
                                    placeholder="Email"
                                    required=true
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    name="password"
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    required=true
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
