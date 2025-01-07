// Modules
mod auth;
mod components;
mod models;
mod pages;

use crate::auth::{validate_signup, SignupCommand, SignupResponse};
use crate::components::Navbar;
use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{FlatRoutes, Route, Router};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provides context that manages the authentication state.
    let (is_authenticated, set_is_authenticated) = signal(false);
    let (username, set_username) = signal::<Option<String>>(None);
    provide_context::<auth::AuthContext>(auth::AuthContext::new(is_authenticated, username));

    // Actions for login, signup, and logout.
    let login: auth::LoginAction = Action::new(move |data: &auth::LoginCommand| {
        let data = data.to_owned();

        set_is_authenticated.set(true);
        set_username.set(Some(data.username.clone()));

        // Simulate a successful login.
        async move { Ok(auth::LoginMessages::Successful) }
    });
    let signup: auth::SignupSignal = Action::new(|data: &SignupCommand| {
        let validation = validate_signup(&data);

        async move {
            if let Err(x) = validation {
                return Ok(SignupResponse::ValidationError(x));
            }
            Ok(SignupResponse::CreateUserError(
                "not implemented yet".into(),
            ))
        }
    });
    let logout: auth::LogoutAction = Action::new(move |_| {
        set_is_authenticated.set(false);
        set_username.set(None);

        async move { true }
    });

    // Redirects to the login page after logout.
    Effect::new(move || {
        let data = logout.value();
        if data.get().unwrap_or_default() {
            let navigate = leptos_router::hooks::use_navigate();
            navigate("/login", leptos_router::NavigateOptions::default());
        }
    });

    view! {
        <Router>
            <nav class="navbar navbar-light">
                <div class="container">
                    <a class="navbar-brand" href="/">
                        "conduit"
                    </a>
                    <ul class="nav navbar-nav pull-xs-right">
                        <Navbar logout />
                    </ul>
                </div>
            </nav>
            <main>
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=path!("/") view=move || view! { <HomePage /> } />
                    <Route path=path!("/article/:slug") view=move || view! { <ArticlePage /> } />
                    <Route path=path!("/login") view=move || view! { <Login login /> } />
                    <Route path=path!("/signup") view=move || view! { <SignupPage signup /> } />
                </FlatRoutes>
            </main>
            <footer>
                <div class="container">
                    <a href="/" class="logo-font">
                        "conduit"
                    </a>
                    <span class="attribution">
                        "An interactive learning project from "
                        <a href="https://thinkster.io">"Thinkster"</a>
                        ". Code &amp; design licensed under MIT."
                    </span>
                </div>
            </footer>
        </Router>
    }
}
