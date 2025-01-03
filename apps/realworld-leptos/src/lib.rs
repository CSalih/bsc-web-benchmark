// Modules
mod components;
mod pages;
mod models;
mod auth;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, Route, FlatRoutes};
use leptos_router::path;
use crate::components::Navbar;
use crate::pages::*;


async fn load_data() -> String {
    let res = reqwest::Client::new()
        .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .unwrap();

    res.text().await.unwrap()
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();


    let (username, set_username) = signal::<Option<String>>(None);
    let logout: auth::LogoutSignal = Action::new(|_| {
        async move { todo!() }
    });
    let login: auth::LoginSignal = Action::new(|_| {
        async move { todo!() }
    });
    let signup: auth::SignupSignal = Action::new(|_| {
        async move { todo!() }
    });

    view! {
        <Router>
            <nav class="navbar navbar-light">
                <div class="container">
                    <a class="navbar-brand" href="/">
                        "conduit"
                    </a>
                    <ul class="nav navbar-nav pull-xs-right">
                        <Navbar logout username />
                    </ul>
                </div>
            </nav>
            <main>
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=path!("/") view=move || view! { <HomePage username /> } />
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