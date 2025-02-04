use leptos::prelude::*;
use leptos_router::params::Params;
use leptos_router::hooks::use_params;
use crate::auth;
use crate::components::ArticlePreviewList;
use crate::models::{Article, Pagination};


#[derive(Params, PartialEq)]
struct ProfilePageParams {
    username: Option<String>,
}

#[component]
#[allow(non_snake_case)]
pub fn ProfilePage() -> impl IntoView {
    let params = use_params::<ProfilePageParams>();
    let auth_context = expect_context::<auth::AuthContext>();

    let Some(user) = auth_context.user.get() else {
        panic!("Not authenticated"); // TODO: redirect to login page
    };

    let (pagination, set_pagination) = signal(Pagination::default());
    let articles_res = LocalResource::new(
        move || Article::load_my_feed(auth_context.access_token.get().unwrap_or_default(), pagination.get())
    );
    let pages = move || {
        let articles_res_opt = articles_res.get();
        if let Some(articles_res) = articles_res_opt.as_deref() {
            let max_page = (articles_res.articles_count as f64
                / pagination.get().get_amount() as f64)
                .ceil() as u32;
            (1..=max_page).collect::<Vec<u32>>()
        } else {
            vec![]
        }
    };

    let (username, _) = signal(user.username());
    let (profile_picture, _) = signal(user.image().unwrap_or_default());
    let (bio, _) = signal(user.bio().unwrap_or_default());

    let is_my_profile = move || {
        let current_path_param = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.username.clone())
            .unwrap_or_default();

        username.get() != current_path_param
    };

    view! {
        <div class="profile-page">
            <div class="user-info">
                <div class="container">
                    <div class="row">
                        <div class="col-xs-12 col-md-10 offset-md-1">
                            <img src=profile_picture class="user-img" />
                            <h4>{username}</h4>
                            <p>{bio}</p>
                            <Show
                                when=move || is_my_profile()
                                fallback=move || {
                                    view! {
                                        <a
                                            href="/settings"
                                            class="btn btn-sm btn-outline-secondary action-btn"
                                        >
                                            <i class="ion-gear-a" />
                                            {" "}
                                            Edit Profile Settings
                                        </a>
                                    }
                                }
                            >
                                <a class="btn btn-sm btn-outline-secondary action-btn">
                                    <i class="ion-plus-round" />
                                    {" "}
                                    Follow
                                    {" "}
                                    {username}
                                </a>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>

            <div class="container">
                <div class="row">
                    <div class="col-xs-12 col-md-10 offset-md-1">
                        <div class="articles-toggle">
                            <ul class="nav nav-pills outline-active">
                                <li class="nav-item">
                                    <a class="nav-link active" href="">
                                        My Articles
                                    </a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="">
                                        Favorited Articles
                                    </a>
                                </li>
                            </ul>
                        </div>

                        <Transition fallback=|| {
                            view! { <p>"Loading articles"</p> }
                        }>
                            {move || {
                                articles_res
                                    .get()
                                    .as_deref()
                                    .map(|articles_res| {
                                        let (articles, _) = signal(articles_res.articles.clone());
                                        view! { <ArticlePreviewList articles=articles /> }
                                    })
                            }}
                        </Transition>

                        <ul class="pagination">
                            <For
                                each=move || pages()
                                key=|x| *x
                                children=move |x| {
                                    let active = x == pagination.get().get_page();
                                    view! {
                                        <li class="page-item" class:active=move || active>
                                            <button
                                                class="page-link"
                                                on:click=move |_| {
                                                    let pagination = pagination.get().clone().set_page(x);
                                                    set_pagination.set(pagination);
                                                }
                                            >
                                                {x}
                                            </button>
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

