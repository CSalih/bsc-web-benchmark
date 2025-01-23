// Modules
mod auth;
mod components;
mod models;
mod pages;
mod utils;

use crate::components::Navbar;
use crate::models::User;
use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{FlatRoutes, Route, Router, ProtectedRoute};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provides context that manages the authentication state.
    let (access_token, set_access_token) = signal::<Option<String>>(None);
    let (user, set_user) = signal::<Option<User>>(None);
    provide_context::<auth::AuthContext>(auth::AuthContext::new(user, access_token));

    let logout: auth::LogoutAction = Action::new(move |_| {
        set_access_token.set(None);
        set_user.set(None);

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
                    <Route
                        path=path!("/login")
                        view=move || view! { <Login set_user access_token set_access_token /> }
                    />
                    <Route path=path!("/signup") view=move || view! { <SignupPage /> } />
                    <ProtectedRoute
                        condition=move || user.get().map(|_| Some(true)).unwrap_or_default()
                        redirect_path=|| "/login"
                        path=path!("/settings")
                        view=move || view! { <SettingsPage update_user=set_user logout /> }
                    />
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
