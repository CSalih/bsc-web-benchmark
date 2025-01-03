use crate::auth::*;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub(crate) fn NavItems(logout: LogoutSignal, username: ReadSignal<Option<String>>) -> impl IntoView {
    let profile_label = move || username.get().unwrap_or_default();
    let profile_href = move || format!("/profile/{}", profile_label());

    view! {
        <li class="nav-item">
            <a class="nav-link" href="/">
                "Home"
            </a>
        </li>
        <Show
            when=move || username.with(Option::is_none)
            fallback=move || {
                view! {
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
                        <button class="nav-link" style="background: none; border: none;">
                            "Logout"
                        </button>
                    </li>
                }
            }
        >
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
        </Show>
    }
}
