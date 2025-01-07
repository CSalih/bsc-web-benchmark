use crate::auth::*;
use leptos::prelude::*;

#[component]
pub fn Navbar(logout: LogoutAction) -> impl IntoView {
    let auth_context = expect_context::<AuthContext>();

    let profile_label = move || auth_context.username.get().unwrap_or_default();
    let profile_href = move || format!("/profile/{}", profile_label());

    view! {
        <li class="nav-item">
            <a class="nav-link" href="/">
                "Home"
            </a>
        </li>
        <Show
            when=move || auth_context.is_authenticated.get()
            fallback=move || {
                view! {
                    <li class="nav-item">
                        <a class="nav-link" href="/login">
                            "Sign in"
                        </a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="/signup">
                            "Sign up"
                        </a>
                    </li>
                }
            }
        >
            <li class="nav-item">
                <a class="nav-link" href="/editor">
                    "New Article"
                </a>
            </li>
            <li class="nav-item">
                <a class="nav-link" href="/settings">
                    "Settings"
                </a>
            </li>
            <li class="nav-item">
                <a class="nav-link" href=profile_href>
                    {profile_label}
                </a>
            </li>
            <li class="nav-item">
                <button
                    class="nav-link"
                    style="background: none; border: none;"
                    on:click=move |_| {
                        logout.dispatch(());
                    }
                >
                    "Logout"
                </button>
            </li>
        </Show>
    }
}
